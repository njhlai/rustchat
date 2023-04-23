import { InputTypes, JoinInput, LeaveInput, PostInput } from "../types/input";

export function join(name: string) {
    return {
        type: InputTypes.Join,
        payload: {
            name: name,
        },
    } as JoinInput;
}

export function leave() {
    return {
        type: InputTypes.Leave,
    } as LeaveInput;
}

export function post(body: string) {
    return {
        type: InputTypes.Post,
        payload: {
            body: body,
        },
    } as PostInput;
}
