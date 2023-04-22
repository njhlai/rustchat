import { InputTypes, JoinInput, LeaveInput } from "../types/input";

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
