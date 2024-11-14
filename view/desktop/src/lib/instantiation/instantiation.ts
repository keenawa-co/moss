import { DisposableStore } from "@/lib/base/lifecycle";
import { SyncDescriptor0 } from "./descriptor";
import { ServiceCollection, ServiceIdentifier } from "./serviceCollection";

export namespace _util {
  export const DI_TARGET = "$di$target";
  export const DI_DEPENDENCIES = "$di$dependencies";

  export const serviceIds = new Map<string, ServiceIdentifier<any>>();

  export function getServiceDependencies(ctor: any): { id: ServiceIdentifier<any>; index: number }[] {
    return ctor[DI_DEPENDENCIES] || [];
  }
}

function storeServiceDependency(id: Function, target: Function, index: number): void {
  if ((target as any)[_util.DI_TARGET] === target) {
    (target as any)[_util.DI_DEPENDENCIES].push({ id, index });
  } else {
    (target as any)[_util.DI_DEPENDENCIES] = [{ id, index }];
    (target as any)[_util.DI_TARGET] = target;
  }
}

export function createDecorator<T>(serviceId: string): ServiceIdentifier<T> {
  if (_util.serviceIds.has(serviceId)) {
    return _util.serviceIds.get(serviceId)!;
  }

  const id = (<any>function (target: Function, _key: string | symbol, index: number) {
    if (arguments.length !== 3) {
      throw new Error("@createDecorator can only be used to decorate a parameter");
    }
    storeServiceDependency(id, target, index);
  }) as ServiceIdentifier<T>;

  id.toString = () => serviceId;
  _util.serviceIds.set(serviceId, id);
  return id;
}

/**
 * A branded service to prevent structural type equivalence.
 */
export type BrandedService = { _serviceBrand: undefined };

/**
 * Accessor used to retrieve services in `invokeFunction`.
 */
export interface ServicesAccessor {
  get<T>(id: ServiceIdentifier<T>): T;
}

/**
 * Given a list of arguments as a tuple, attempt to extract the leading, non-service arguments
 * to their own tuple.
 */
export type GetLeadingNonServiceArgs<TArgs extends any[]> = TArgs extends []
  ? []
  : TArgs extends [...infer TFirst, BrandedService]
    ? GetLeadingNonServiceArgs<TFirst>
    : TArgs;

export const IInstantiationService = createDecorator<IInstantiationService>("instantiationService");

export interface IInstantiationService {
  readonly _serviceBrand: undefined;

  /**
   * Synchronously creates an instance that is denoted by the descriptor
   */
  createInstance<T>(descriptor: SyncDescriptor0<T>): T;
  createInstance<Ctor extends new (...args: any[]) => unknown, R extends InstanceType<Ctor>>(
    ctor: Ctor,
    ...args: GetLeadingNonServiceArgs<ConstructorParameters<Ctor>>
  ): R;

  /**
   * Creates a child of this service which inherits all current services
   * and adds/overwrites the given services.
   *
   * NOTE that the returned child is `disposable` and should be disposed when not used
   * anymore. This will also dispose all the services that this service has created.
   */
  createChild(services: ServiceCollection, store?: DisposableStore): IInstantiationService;

  /**
   * Calls a function with a service accessor.
   */
  invokeFunction<R, TS extends any[] = []>(fn: (accessor: ServicesAccessor, ...args: TS) => R, ...args: TS): R;

  /**
   * Disposes this instantiation service.
   *
   * - Will dispose all services that this instantiation service has created.
   * - Will dispose all its children but not its parent.
   * - Will NOT dispose services-instances that this service has been created with
   * - Will NOT dispose consumer-instances this service has created
   */
  dispose(): void;
}
