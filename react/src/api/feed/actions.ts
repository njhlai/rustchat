import { ActivityTimeStamp, ActivityTypes, Feed, LoadActivity, Message, MessageActivity, UserJoinedActivity, UserLeftActivity } from "../types/data";
import { CurrentStateOutput } from "../types/output";

export function currentState(output: CurrentStateOutput) {
    return {
        users: output.payload.users,
        activities: output.payload.messages.map((msg) =>
            message(msg)
        ),
    } as Feed;
}

export function load() {
    return {
        type: ActivityTypes.Load,
    } as LoadActivity;
}

export function message(msg: Message) {
    return {
        type: ActivityTypes.Message,
        event: msg,
    } as MessageActivity;
}

export function userJoin(activityTimeStamp: ActivityTimeStamp) {
    return {
        type: ActivityTypes.UserJoined,
        event: activityTimeStamp,
    } as UserJoinedActivity;
}

export function userLeft(activityTimeStamp: ActivityTimeStamp) {
    return {
        type: ActivityTypes.UserLeft,
        event: activityTimeStamp,
    } as UserLeftActivity;
}
