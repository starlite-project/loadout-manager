import type { StorybookViteConfig } from "@storybook/builder-vite";
import { mergeConfig } from "vite";

const config: StorybookViteConfig = {
	stories: [
		"../src/**/*.stories.mdx",
		"../src/**/*.stories@(js|jsx|ts|tsx)"
	],
	addons: [
		"@storybook/addon-links",
		"@storybook/addon-essentials",
		"@storybook/addon-interactions"
	],
	framework: '@storybook/react',
	core: {
		builder: '@storybook/builder-vite'
	},
	features: {
		storyStoreV7: true,
	},
	async viteFinal(config) {
		const { default: mainConfig } = await import('../vite.config');
		return mergeConfig(config, {
			...mainConfig
		});
	}
};

export default config;