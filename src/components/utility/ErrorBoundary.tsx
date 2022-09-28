import React from 'react';

interface Props {
	name: string;
	children?: React.ReactNode;
}

interface State {
	error?: Error;
}

export class ErrorBoundary extends React.Component<Props, State> {
	public state: State = {};

	public componentDidCatch(error: Error): void {
		this.setState({ error });
	}

	public render(): React.ReactNode {
		const { error } = this.state;
		const { children } = this.props;

		if (error) {
			return (
				<div>
					<p>
						An error has occurred: {error.name}: {error.message}
					</p>
					{error.stack ? <p>{error.stack}</p> : null}
				</div>
			);
		}

		return children;
	}
}
