import { useDispatch } from 'react-redux';
import type { LoadoutManagerDispatch } from './types';

export const useThunkDispatch = (): LoadoutManagerDispatch => useDispatch<LoadoutManagerDispatch>();
