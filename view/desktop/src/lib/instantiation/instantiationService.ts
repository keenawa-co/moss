import { DisposableStore, dispose, isDisposable } from "@/lib/base/lifecycle";
import { SyncDescriptor, SyncDescriptor0 } from "./descriptor";
import { _util, GetLeadingNonServiceArgs, IInstantiationService, ServicesAccessor } from "./instantiation";
import { ServiceCollection, ServiceIdentifier } from "./serviceCollection";
import { Graph } from "./graph";
import { illegalState } from "@/lib/base/errors";

class CyclicDependencyError extends Error {
  constructor(graph: Graph<any>) {
    super("cyclic dependency between services");
    this.message = graph.findCycleSlow() ?? `UNABLE to detect cycle, dumping graph: \n${graph.toString()}`;
  }
}

export class InstantiationService implements IInstantiationService {
  declare readonly _serviceBrand: undefined;

  private readonly _services: ServiceCollection;
  private readonly _strict: boolean;
  private readonly _parent?: InstantiationService;

  private _isDisposed = false;
  private readonly _servicesToDispose = new Set<any>();
  private readonly _childServices = new Set<InstantiationService>();

  constructor(
    services: ServiceCollection = new ServiceCollection(),
    strict: boolean = false,
    parent?: InstantiationService
  ) {
    this._services = services;
    this._strict = strict;
    this._parent = parent;

    this._services.set(IInstantiationService, this);
  }

  dispose(): void {
    if (!this._isDisposed) {
      this._isDisposed = true;
      // dispose all child services
      dispose(this._childServices);
      this._childServices.clear();

      // dispose all services created by this service
      for (const candidate of this._servicesToDispose) {
        if (isDisposable(candidate)) {
          candidate.dispose();
        }
      }
      this._servicesToDispose.clear();
    }
  }

  private _throwIfDisposed(): void {
    if (this._isDisposed) {
      throw new Error("InstantiationService has been disposed");
    }
  }

  createChild(services: ServiceCollection, store?: DisposableStore): IInstantiationService {
    this._throwIfDisposed();
    //eslint-disable-next-line
    const that = this;
    const result = new (class extends InstantiationService {
      override dispose(): void {
        that._childServices.delete(result);
        super.dispose();
      }
    })(services, this._strict, this);
    this._childServices.add(result);

    store?.add(result);
    return result;
  }

  invokeFunction<R, TS extends any[] = []>(fn: (accessor: ServicesAccessor, ...args: TS) => R, ...args: TS): R {
    this._throwIfDisposed();

    let done = false;
    try {
      const accessor: ServicesAccessor = {
        get: <T>(id: ServiceIdentifier<T>) => {
          if (done) {
            throw illegalState("Service accessor is only valid during the invocation of its target method");
          }

          const result = this._getOrCreateServiceInstance(id);
          if (!result) {
            throw new Error(`[invokeFunction] Unknown service '${id}'`);
          }
          return result;
        },
      };
      return fn(accessor, ...args);
    } finally {
      done = true;
    }
  }

  createInstance<T>(descriptor: SyncDescriptor0<T>): T;
  createInstance<Ctor extends new (...args: any[]) => unknown, R extends InstanceType<Ctor>>(
    ctor: Ctor,
    ...args: GetLeadingNonServiceArgs<ConstructorParameters<Ctor>>
  ): R;
  createInstance(ctorOrDescriptor: any | SyncDescriptor<any>, ...rest: any[]): unknown {
    this._throwIfDisposed();

    if (ctorOrDescriptor instanceof SyncDescriptor) {
      return this._createInstance(ctorOrDescriptor.ctor, ctorOrDescriptor.staticArguments.concat(rest));
    } else {
      return this._createInstance(ctorOrDescriptor, rest);
    }
  }

  private _createInstance<T>(ctor: any, args: any[] = []): T {
    const serviceDependencies = _util.getServiceDependencies(ctor).sort((a, b) => a.index - b.index);
    const serviceArgs: any[] = [];

    for (const dependency of serviceDependencies) {
      const service = this._getOrCreateServiceInstance(dependency.id);
      if (!service) {
        this._throwIfStrict(`[createInstance] ${ctor.name} depends on UNKNOWN service ${dependency.id}.`, false);
      }

      serviceArgs.push(service);
    }

    const firstServiceArgPos = serviceDependencies.length > 0 ? serviceDependencies[0].index : args.length;

    if (args.length !== firstServiceArgPos) {
      const delta = firstServiceArgPos - args.length;
      if (delta > 0) {
        args = args.concat(new Array(delta));
      } else {
        args = args.slice(0, firstServiceArgPos);
      }
    }

    return Reflect.construct(ctor, args.concat(serviceArgs));
  }

  private _setCreatedServiceInstance<T>(id: ServiceIdentifier<T>, instance: T): void {
    if (this._services.get(id) instanceof SyncDescriptor) {
      this._services.set(id, instance);
    } else if (this._parent) {
      this._parent._setCreatedServiceInstance(id, instance);
    } else {
      throw new Error("Illegal state - setting UNKNOWN service instance");
    }
  }

  private _getServiceInstanceOrDescriptor<T>(id: ServiceIdentifier<T>): T | SyncDescriptor<T> | undefined {
    const instanceOrDesc = this._services.get(id);
    if (!instanceOrDesc && this._parent) {
      return this._parent._getServiceInstanceOrDescriptor(id);
    } else {
      return instanceOrDesc;
    }
  }

  private _getOrCreateServiceInstance<T>(id: ServiceIdentifier<T>): T {
    const thing = this._getServiceInstanceOrDescriptor(id);
    if (thing instanceof SyncDescriptor) {
      return this._safeCreateAndCacheServiceInstance(id, thing);
    } else if (thing !== undefined) {
      return thing;
    } else {
      throw new Error(`[getOrCreateServiceInstance] Unknown service '${id}'`);
    }
  }

  private readonly _activeInstantiations = new Set<ServiceIdentifier<any>>();

  private _safeCreateAndCacheServiceInstance<T>(id: ServiceIdentifier<T>, desc: SyncDescriptor<T>): T {
    if (this._activeInstantiations.has(id)) {
      throw new Error(`Illegal state - cyclic dependency detected while instantiating service '${id}'`);
    }

    this._activeInstantiations.add(id);
    try {
      return this._createAndCacheServiceInstance(id, desc);
    } finally {
      this._activeInstantiations.delete(id);
    }
  }

  private _createAndCacheServiceInstance<T>(id: ServiceIdentifier<T>, desc: SyncDescriptor<T>): T {
    const graph = new Graph<ServiceIdentifier<any>>((id) => id.toString());
    const stack = [{ id, desc }];
    const seen = new Set<ServiceIdentifier<any>>();

    while (stack.length) {
      const item = stack.pop()!;
      if (seen.has(item.id)) {
        continue;
      }
      seen.add(item.id);

      graph.lookupOrInsertNode(item.id);

      for (const dependency of _util.getServiceDependencies(item.desc.ctor)) {
        const instanceOrDesc = this._getServiceInstanceOrDescriptor(dependency.id);
        if (instanceOrDesc instanceof SyncDescriptor) {
          graph.insertEdge(item.id, dependency.id);
          stack.push({ id: dependency.id, desc: instanceOrDesc });
        }
      }
    }

    if (graph.findCycleSlow()) {
      throw new CyclicDependencyError(graph);
    }

    for (const nodeId of graph.topologicalSort()) {
      const instanceOrDesc = this._getServiceInstanceOrDescriptor(nodeId);
      if (instanceOrDesc instanceof SyncDescriptor) {
        const instance = this._createServiceInstanceWithOwner(
          nodeId,
          instanceOrDesc.ctor,
          instanceOrDesc.staticArguments,
          instanceOrDesc.supportsDelayedInstantiation
        );
        this._setCreatedServiceInstance(nodeId, instance);
      }
    }

    const result = this._getServiceInstanceOrDescriptor(id);
    if (result instanceof SyncDescriptor) {
      throw new Error(`Failed to create service instance for '${id}'`);
    }
    return result!;
  }

  private _createServiceInstanceWithOwner<T>(
    id: ServiceIdentifier<T>,
    ctor: any,
    args: any[] = [],
    supportsDelayedInstantiation: boolean
  ): T {
    if (this._services.get(id) instanceof SyncDescriptor) {
      return this._createServiceInstance(ctor, args);
    } else if (this._parent) {
      return this._parent._createServiceInstanceWithOwner(id, ctor, args, supportsDelayedInstantiation);
    } else {
      throw new Error(`Illegal state - creating UNKNOWN service instance '${ctor.name}'`);
    }
  }

  private _createServiceInstance<T>(ctor: any, args: any[] = []): T {
    const instance = this._createInstance<T>(ctor, args);

    if (isDisposable(instance)) {
      this._servicesToDispose.add(instance);
    }

    return instance;
  }

  private _throwIfStrict(msg: string, printWarning: boolean): void {
    if (printWarning) {
      console.warn(msg);
    }
    if (this._strict) {
      throw new Error(msg);
    }
  }
}
