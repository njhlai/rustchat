"use client";

import { ChangeEvent, FormEvent, useState } from "react";

import { useAppDispatch, useAppSelector } from "../app/hooks";
import { post } from "../api/user/actions";
import { ActivityTypes } from "../api/types/data";

import styles from "../styles/layout.module.css";

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
        <div className={styles.feed}>
            <ul className={styles.chat}>
                {acitivities.map((activity) => {
                    switch (activity.type) {
                        case ActivityTypes.Load:
                            return (
                                <li style={{ margin: 15, textAlign: "center" }}>
                                    {" "}
                                    Hello{" "}
                                    <span className={styles.myself}>
                                        {username}
                                    </span>
                                    !
                                </li>
                            );

                            break;
                        case ActivityTypes.Message:
                            return (
                                <li key={activity.event.id.toString()}>
                                    <span className={styles.id}>
                                        {activity.event.sender.toString()}
                                    </span>
                                    : {activity.event.body}{" "}
                                    <span className={styles.timestamp}>
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
                                    <span className={styles.id}>
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
                                    <span className={styles.id}>
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
                    className={styles.message_input}
                    type="text"
                    placeholder="Enter your message"
                    onChange={handleChange}
                    value={message}
                />
            </form>
        </div>
    );
}
