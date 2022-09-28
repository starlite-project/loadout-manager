export interface Application {
	applicationId: number;
	name: string;
	redirect_url: string;
	link: string;
	scope: string;
	origin?: string;
	status: number;
}
