let Kind;
(function(Kind) {
  Kind[(Kind["Notification"] = 0)] = "Notification";
  Kind[(Kind["ProjectRegistration"] = 1)] = "ProjectRegistration";
})(Kind || (Kind = {}));
let state = [];
function update(state, msg) {
  switch (msg.kind) {
    case Kind.Notification:
      console.log("notification");
      console.log(msg.msg);
      break;
    case Kind.ProjectRegistration:
      console.log("project registration");
      console.log(msg.project);
      break;
  }
  return state;
}
export function emit(msg) {
  console.log(msg);
  state = update(state, msg);
}
emit({ kind: Kind.ProjectRegistration, project: "upstream" });
// # sourceMappingURL=event.js.map
