export enum InputTypes {
    Join = "Join",
    Leave = "Leave",
    Post = "Post",
}

export interface JoinInput {
    type: InputTypes.Join;
    payload: {
        name: string;
    };
}

export interface LeaveInput {
    type: InputTypes.Leave;
}

export interface PostInput {
    type: InputTypes.Post;
    payload: {
        body: string;
    };
}

export type Input = JoinInput | LeaveInput | PostInput;
