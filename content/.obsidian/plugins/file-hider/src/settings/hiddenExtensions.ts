import { Setting } from "obsidian";
import FileHider from "../main";

export class HideExtensions {

    public static create(plugin: FileHider, container: HTMLElement) {
		return new Setting(container)
		.setName('List extensions to hide')
		.setDesc('String with extensions splitted by space')
		.addText(text => {
			text
			.setValue(plugin.settings.hiddenExtensions)
            .setPlaceholder('Enter your extensions to hide')
			.onChange(async (value) => {
                plugin.settings.hiddenExtensions = value;
				await plugin.saveSettings();
			});
		});
	};

};
