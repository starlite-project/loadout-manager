import React from 'react';
import type { FC } from 'react';
import type { GeneralUser } from '../models';
import useSWR from 'swr';

export const User: FC = () => {
	const { data, error } = useSWR<GeneralUser>('get_current_user');

	if (error) {
		throw error;
	}

	return <div>{`Hello, ${data!.displayName}`}</div>;
};

export default User;
