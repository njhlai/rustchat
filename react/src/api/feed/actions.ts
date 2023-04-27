import { ActivityTimeStamp, ActivityTypes, LoadActivity, Message, MessageActivity, UserJoinedActivity, UserLeftActivity } from "../types/data";

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

export function userjoin(activityTimeStamp: ActivityTimeStamp) {
    return {
        type: ActivityTypes.UserJoined,
        event: activityTimeStamp,
    } as UserJoinedActivity;
}

export function userleft(activityTimeStamp: ActivityTimeStamp) {
    return {
        type: ActivityTypes.UserLeft,
        event: activityTimeStamp,
    } as UserLeftActivity;
}
