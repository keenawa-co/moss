import { entry_preview_exports } from './chunk-WUESIYTI.mjs';
import './chunk-JSBTCGGE.mjs';
import { global } from '@storybook/global';
import { setProjectAnnotations as setProjectAnnotations$1, composeStory as composeStory$1, composeStories as composeStories$1 } from '@storybook/preview-api';

var{window:globalWindow}=global;globalWindow&&(globalWindow.STORYBOOK_ENV="react");function setProjectAnnotations(projectAnnotations){setProjectAnnotations$1(projectAnnotations);}var INTERNAL_DEFAULT_PROJECT_ANNOTATIONS=entry_preview_exports;function composeStory(story,componentAnnotations,projectAnnotations,exportsName){return composeStory$1(story,componentAnnotations,projectAnnotations,INTERNAL_DEFAULT_PROJECT_ANNOTATIONS,exportsName)}function composeStories(csfExports,projectAnnotations){return composeStories$1(csfExports,projectAnnotations,composeStory)}typeof module<"u"&&module?.hot?.decline();

export { INTERNAL_DEFAULT_PROJECT_ANNOTATIONS, composeStories, composeStory, setProjectAnnotations };
