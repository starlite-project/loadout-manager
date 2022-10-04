type Subscription<T> = (value: T) => void;
type Unsubscribe = () => void;

export class EventBus<T> {
    private _subscriptions = new Set<Subscription<T>>();

    public next(value: T): void {
        for (const subscription of this._subscriptions) {
            subscription(value);
        }
    }

    public subscribe(callback: Subscription<T>): Unsubscribe {
        this._subscriptions.add(callback);
        return () => this._subscriptions.delete(callback);
    }
}

export class Observable<T> {
    private _event = new EventBus<T>();

    public getCurrentValue = (): T => this._value;

    public subscribe = (callback: Subscription<T>): Unsubscribe => this._event.subscribe(callback);

    public constructor(private _value: T) { }

    public next(value: T): void {
        this._value = value;
        this._event.next(value);
    }
}