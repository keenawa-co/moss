export class Node<T> {
  readonly incoming = new Map<string, Node<T>>();
  readonly outgoing = new Map<string, Node<T>>();

  constructor(
    readonly key: string,
    readonly data: T
  ) {}
}

export class Graph<T> {
  private readonly _nodes = new Map<string, Node<T>>();

  constructor(private readonly _hashFn: (element: T) => string) {}

  roots(): Node<T>[] {
    const ret: Node<T>[] = [];
    for (const node of this._nodes.values()) {
      if (node.outgoing.size === 0) {
        ret.push(node);
      }
    }
    return ret;
  }

  insertEdge(from: T, to: T): void {
    const fromNode = this.lookupOrInsertNode(from);
    const toNode = this.lookupOrInsertNode(to);

    fromNode.outgoing.set(toNode.key, toNode);
    toNode.incoming.set(fromNode.key, fromNode);
  }

  removeNode(data: T): void {
    const key = this._hashFn(data);
    this._nodes.delete(key);
    for (const node of this._nodes.values()) {
      node.outgoing.delete(key);
      node.incoming.delete(key);
    }
  }

  lookupOrInsertNode(data: T): Node<T> {
    const key = this._hashFn(data);
    let node = this._nodes.get(key);

    if (!node) {
      node = new Node(key, data);
      this._nodes.set(key, node);
    }

    return node;
  }

  lookup(data: T): Node<T> | undefined {
    return this._nodes.get(this._hashFn(data));
  }

  isEmpty(): boolean {
    return this._nodes.size === 0;
  }

  toString(): string {
    const data: string[] = [];
    for (const [key, value] of this._nodes) {
      data.push(
        `${key}\n\t(-> incoming)[${[...value.incoming.keys()].join(", ")}]\n\t(outgoing ->)[${[...value.outgoing.keys()].join(",")}]\n`
      );
    }
    return data.join("\n");
  }

  /**
   * Performs a topological sort on the graph and returns an array of node data in topologically sorted order.
   * Throws an error if the graph contains a cycle.
   */
  topologicalSort(): T[] {
    const sorted: T[] = [];
    const nodes = new Map<string, Node<T>>();

    // Clone the nodes and initialize in-degree map
    const inDegreeMap = new Map<string, number>();
    for (const [key, node] of this._nodes) {
      nodes.set(key, node);
      inDegreeMap.set(key, node.incoming.size);
    }

    // Initialize the queue with nodes that have no incoming edges (in-degree = 0)
    const queue: Node<T>[] = [];
    for (const [key, inDegree] of inDegreeMap) {
      if (inDegree === 0) {
        queue.push(nodes.get(key)!);
      }
    }

    while (queue.length > 0) {
      const node = queue.shift()!;
      sorted.push(node.data);

      // For each node m with an edge from current node to m
      for (const [mKey, mNode] of node.outgoing) {
        // Remove the edge from current node to m
        mNode.incoming.delete(node.key);

        // Decrease in-degree of mNode
        const inDegree = inDegreeMap.get(mKey)! - 1;
        inDegreeMap.set(mKey, inDegree);

        // If mNode has no other incoming edges, add it to the queue
        if (inDegree === 0) {
          queue.push(mNode);
        }
      }
      // Remove all outgoing edges from current node
      node.outgoing.clear();
    }

    // If there are nodes with in-degree > 0, then there is a cycle
    for (const inDegree of inDegreeMap.values()) {
      if (inDegree > 0) {
        throw new Error("Graph has at least one cycle. Topological sort not possible.");
      }
    }

    return sorted;
  }

  /**
   * This is a brute-force and slow method and should only be used
   * for troubleshooting purposes.
   */
  findCycleSlow() {
    for (const [id, node] of this._nodes) {
      const seen = new Set<string>([id]);
      const res = this._findCycle(node, seen);
      if (res) {
        return res;
      }
    }
    return undefined;
  }

  private _findCycle(node: Node<T>, seen: Set<string>): string | undefined {
    for (const [id, outgoing] of node.outgoing) {
      if (seen.has(id)) {
        return [...seen, id].join(" -> ");
      }
      seen.add(id);
      const value = this._findCycle(outgoing, seen);
      if (value) {
        return value;
      }
      seen.delete(id);
    }
    return undefined;
  }
}
