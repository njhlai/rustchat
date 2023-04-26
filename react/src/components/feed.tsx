"use client";

import { CSSProperties, ChangeEvent, FormEvent, useState } from "react";

import { useAppDispatch, useAppSelector } from "../app/hooks";
import { post } from "../api/user/actions";
import { ActivityTypes } from "../api/types/data";

const styles = {
    chat: {
        listStyle: "none",
        paddingLeft: 10,
        textAlign: "left",
    },
    id: {
        color: "yellow",
    },
    name: {
        color: "cyan",
    },
    myid: {
        color: "orange",
    },
    myself: {
        color: "green",
    },
    messageInput: {
        textAlign: "left",
        width: "98%",
    },
    timestamp: {
        fontStyle: "italic",
        float: "right",
        color: "red",
    },
} as Record<string, CSSProperties>;

export default function Feed({ username }: { username: string }) {
    const acitivities = useAppSelector((state) => state.feed.activities);

    const [message, setMessage] = useState("");

    const dispatch = useAppDispatch();

    function handleChange(event: ChangeEvent<HTMLInputElement>) {
        setMessage(event.target.value);
    }

    function handleSubmit(event: FormEvent<HTMLFormElement>) {
        event.preventDefault();
        dispatch(post(message));
        setMessage("");
    }

    return (
        <>
            <ul style={styles.chat}>
                {acitivities.map((activity) => {
                    switch (activity.type) {
                        case ActivityTypes.Load:
                            return (
                                <li style={{ margin: 15, textAlign: "center" }}>
                                    {" "}
                                    Hello{" "}
                                    <span style={styles.myself}>
                                        {username}
                                    </span>
                                    !
                                </li>
                            );

                            break;
                        case ActivityTypes.Message:
                            return (
                                <li key={activity.event.id.toString()}>
                                    <span style={styles.id}>
                                        {activity.event.sender.toString()}
                                    </span>
                                    : {activity.event.body}{" "}
                                    <span style={styles.timestamp}>
                                        {activity.event.timestamp}
                                    </span>
                                </li>
                            );

                            break;
                        case ActivityTypes.UserJoined:
                            return (
                                <li
                                    style={{
                                        margin: 15,
                                        textAlign: "center",
                                        fontStyle: "italic",
                                    }}
                                >
                                    <span style={styles.id}>
                                        {activity.event.user.id.toString()}
                                    </span>
                                    {" joined the server @ "}
                                    <span style={{ color: "red" }}>
                                        {activity.event.timestamp}
                                    </span>
                                </li>
                            );

                            break;
                        case ActivityTypes.UserLeft:
                            return (
                                <li
                                    style={{
                                        margin: 15,
                                        textAlign: "center",
                                        fontStyle: "italic",
                                    }}
                                >
                                    <span style={styles.id}>
                                        {activity.event.user.id.toString()}
                                    </span>
                                    {" left the server! "}
                                    <span style={{ color: "red" }}>
                                        {activity.event.timestamp}
                                    </span>
                                </li>
                            );

                            break;
                        default:
                            return <></>;

                            break;
                    }
                })}
            </ul>
            <form onSubmit={handleSubmit}>
                <input
                    style={styles.messageInput}
                    type="text"
                    placeholder="Enter your message"
                    onChange={handleChange}
                    value={message}
                />
            </form>
        </>
    );
}
