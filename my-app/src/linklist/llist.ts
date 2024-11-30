class Node {
    private _value: number
    private _next: Node | null
    constructor(value: number) {
        this._value = value;
        this._next = null;
    }
    get value(): number {
        return this._value;
    }
    set value(value: number) {
        this._value = value;
    }
    get next(): Node | null {
        return this._next;
    }
    set next(next: Node | null) {
        this._next = next;
    }
}

class LinkedList {
    private head: Node | null;
    // private tail: Node | null;
    // private length: number;

    constructor(value: number) {
        const node = new Node(value);
        this.head = node
        // this.tail = node;
        // this.length = 1;
    }
    printList(): void {

    }
}

const ll = new LinkedList(4);
ll.printList()