import { Base } from "./base";

export interface RawApplication {
	applicationId: number;
	name: string;
	redirectUrl: string;
	link: string;
	scope: string;
	origin?: string;
	status: number;
}

export class Application extends Base<RawApplication> { }