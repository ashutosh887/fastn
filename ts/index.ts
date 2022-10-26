
window.ftd = (function() {
    let ftd_data: any = {};
    let exports: Partial<Export> = {};

    exports.init = function (id: string, data: string) {
        let element = document.getElementById(data);
        if (!!element) {
            ftd_data[id] = JSON.parse(element.innerText);
            // window.ftd.post_init();
        }
    };


    function handle_event(evt: Event, id: string, action: Action, obj: Element) {
        console.log(id, action);
        console.log(action.name);

        let function_arguments: FunctionArgument[] = [];
        let argument: keyof typeof action.values;
        for (argument in action.values) {
            if (action.values.hasOwnProperty(argument)) {
                if (typeof action.values[argument] === 'object') {
                    function_arguments.push(<FunctionArgument>action.values[argument]);
                } else {
                    function_arguments.push({
                        "value": resolve_reference(<string>action.values[argument], ftd_data[id]),
                        "reference": <string>action.values[argument]
                    });
                }
            }
        }

        window[action.name](...function_arguments);
        change_value(function_arguments, ftd_data[id]);
        window["node_change_" + id](ftd_data[id]);
    }

    exports.handle_event = function (evt: Event, id: string, event: string, obj: Element) {
        console_log(id, event);
        let actions = JSON.parse(event);
        for (const action in actions) {
            handle_event(evt, id, actions[action], obj);
        }
    };

    return exports;
})();
