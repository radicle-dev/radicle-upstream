import { readable, writable } from "svelte/store"

import * as event from '../lib/event'
import * as project from '../lib/project'

export const projectNameStore = writable(null)

const projectsState: project.Project[] = [];

export const projects = readable(projectsState, set => {
  event.emit({
    kind: event.Kind.Project,
    msg: {
      kind: project.Kind.FetchList,
    },
  });

  return (): void => set([])
})
