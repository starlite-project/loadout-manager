import type { IconDefinition, IconName, IconPrefix } from '@fortawesome/fontawesome-svg-core';

export const makeCustomIcon = (
    name: string,
    width: number,
    height: number,
    pathData: string
): IconDefinition => ({
    iconName: `inventoryManager${name}` as unknown as IconName,
    prefix: 'inventoryManager' as IconPrefix,
    icon: [width, height, [], '', pathData]
})