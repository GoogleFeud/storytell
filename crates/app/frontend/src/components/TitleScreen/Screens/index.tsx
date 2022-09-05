import { StoriesScreen } from "./Stories";

export const Screens = {
    components: [<StoriesScreen />, <p>Settings</p>, <p>Guides</p>],
    stories: 0,
    settings: 1,
    guides: 2
} as const;