import type React from 'react';
import type { FunctionComponent } from 'react';
import type { Settings } from './initial-settings';
import lodashMap from 'lodash/map';


interface Props {
    label: string;
    value: string | number;
    options: {
        name?: string;
        value: string | number;
    }[];
    name: keyof Settings;
    onChange: React.ChangeEventHandler<HTMLSelectElement>;
}

export const Select: FunctionComponent<Props> = ({ label, value, name, onChange, options }) => {
    return (
        <div className='setting horizontal'>
            <label htmlFor={name}>{label}</label>
            <select name={name} value={value} required={true} onChange={onChange}>
                {options.map((option) => (
                    <option key={option.value} value={option.value}>
                        {option.name ?? option.value}
                    </option>
                ))}
            </select>
        </div>
    )
}

export const mapToOptions = (map: { [key: string]: string }): { name: string, value: string }[] => {
    return lodashMap(map, (value, key) => ({
        name: value,
        value: key,
    }))
}

export default Select;