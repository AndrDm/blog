import { VisibilityToggleCommand } from './commands/toggleVisibility';
import { VisibilityToggleSetting } from './settings/hiddenToggle';
import { HideExtensions } from './settings/hiddenExtensions';
import { App, Plugin, PluginSettingTab, TFolder } from 'obsidian';
import { ManageHiddenPaths } from './settings/manageHiddenPaths';
import { changePathVisibility } from './utils';

interface FileHiderSettings {
	hidden: boolean;
	hiddenList: string[];
	hiddenExtensions: string;
};



export default class FileHider extends Plugin {
	settings: FileHiderSettings = {
		hidden: true,
		hiddenList: [],
		hiddenExtensions: ''
	};

	style: CSSStyleSheet|null = null;

	async onload() {
		await this.loadSettings();

		this.registerEvent(
			this.app.workspace.on(`file-menu`, (menu, file) => {
				if (file instanceof TFolder) {
					menu.addItem((i) => {
						if (this.settings.hiddenList.includes(file.path)) {
							i.setTitle(`Unhide Folder`)
							.setIcon(`eye`)
							.onClick(() => {
								this.unhidePath(file.path);
							});
						} else {
							i.setTitle(`Hide Folder`)
							.setIcon(`eye-off`)
							.onClick(() => {
								changePathVisibility(file.path, this.settings.hidden);
								this.settings.hiddenList.push(file.path);
								this.saveSettings();
							});
						};
					});
				} else {
					menu.addItem((i) => {
						if (this.settings.hiddenList.includes(file.path)) {
							i.setTitle(`Unhide File`)
							.setIcon(`eye`)
							.onClick((e) => {
								this.unhidePath(file.path);
							});
						} else {
							i.setTitle(`Hide File`)
							.setIcon(`eye-off`)
							.onClick((e) => {
								changePathVisibility(file.path, this.settings.hidden);
								this.settings.hiddenList.push(file.path);
								this.saveSettings();
							});
						};
					});
				};
			})
		);

		// this.app.workspace.onLayoutReady(await doScan());
		this.app.workspace.onLayoutReady(this.onLayoutReady.bind(this));
		new VisibilityToggleCommand(this);
		this.addSettingTab(new FileHiderSettingsTab(this.app, this));
	};

	async onLayoutReady():Promise<void> {
		await new Promise(f => setTimeout(f, 200));
		const folders_items = document.querySelectorAll<HTMLElement>('.tree-item .nav-folder');
		for (const folder_item of folders_items) {
			folder_item.addEventListener("click", (e: MouseEvent) => {this.doScan()});
		}
		this.doScan();
	}

	doScan() {
		setTimeout(() => {
			for (const path of this.settings.hiddenList) {
				changePathVisibility(path, this.settings.hidden);
			};
			const exts: string[] = [];
			for (const ex of this.settings.hiddenExtensions.trim().split(' ')) {
				exts.push(ex.toLowerCase());
			}
			const files = this.app.vault.getFiles();
			for (let i = 0; i < files.length; i++) {
				if (exts.indexOf(files[i].extension.toLowerCase()) > -1) {
					changePathVisibility(files[i].path, this.settings.hidden);
				}
			}

		}
		, 100)
	}

	/*
	Loads the config settings, with defaults created where needed.
	*/
	async loadSettings() {
		this.settings = Object.assign({}, this.settings, await this.loadData());
	};

	/* Saves the setting data */
	async saveSettings() {
		await this.saveData(this.settings);
	};

	/*
	Enables/Disables the file visibility based. (gets the stylesheet if needed)
	*/
	toggleVisibility() {
		this.settings.hidden = !this.settings.hidden;
		for (const path of this.settings.hiddenList) {
			changePathVisibility(path, this.settings.hidden);
		};
		this.saveSettings();
	};

	unhidePath(path: string) {
		let i = this.settings.hiddenList.indexOf(path);
		this.settings.hiddenList.splice(i, 1);
		changePathVisibility(path, false);
		this.saveSettings();
	};
};


/**
 * All of the settings for the FileHider
 */
class FileHiderSettingsTab extends PluginSettingTab {
	plugin: FileHider;

	constructor(app: App, plugin: FileHider) {
		super(app, plugin);
		this.plugin = plugin;
	};

	display(): void {
		const { containerEl: container } = this;

		container.empty();
		VisibilityToggleSetting.create(this.plugin, container);
		ManageHiddenPaths.create(this.plugin, container);
		HideExtensions.create(this.plugin, container);
	};
};
