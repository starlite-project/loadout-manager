export class Base<T> {
    public constructor(protected _data: T) { }
}

export interface IconURL {
    iconURL(): URL;
}