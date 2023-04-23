"use client";

import { CSSProperties, ChangeEvent, FormEvent, useState } from "react";

import { useAppDispatch, useAppSelector } from "../app/hooks";
import { post } from "../api/user/actions";

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
    const prevMessages = useAppSelector((state) => state.feed.prevMessages);
    const messages = useAppSelector((state) => state.feed.currMessages);

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
                {prevMessages.map((msg) => {
                    return (
                        <li key={msg.id.toString()}>
                            <span style={styles.id}>
                                {msg.sender.toString()}
                            </span>
                            : {msg.body}{" "}
                            <span style={styles.timestamp}>
                                {msg.timestamp}
                            </span>
                        </li>
                    );
                })}
            </ul>
            <p>
                Hello <span style={styles.myself}>{username}</span>!
            </p>
            <ul style={styles.chat}>
                {messages.map((msg) => {
                    return (
                        <li key={msg.id.toString()}>
                            <span style={styles.id}>
                                {msg.sender.toString()}
                            </span>
                            : {msg.body}{" "}
                            <span style={styles.timestamp}>
                                {msg.timestamp}
                            </span>
                        </li>
                    );
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
