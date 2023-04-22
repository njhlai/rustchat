"use client";

import { CSSProperties, ChangeEvent, FormEvent, useState } from "react";

import { join } from "../api/user/actions";
import { useAppDispatch } from "../app/hooks";

const styles = {
    joinButton: {},
    login: {
        display: "flex",
        flexDirection: "column",
        marginLeft: 10,
        marginRight: 10,
        textAlign: "inherit",
    },
    usernameInput: {
        textAlign: "center",
    },
} as Record<string, CSSProperties>;

export default function Login() {
    const [name, setName] = useState("");
    const isValidName = (name: string) => name.length >= 4;

    const dispatch = useAppDispatch();

    function handleNameChange(event: ChangeEvent<HTMLInputElement>) {
        setName(event.target.value);
    }

    function handleJoin(event: FormEvent) {
        event.preventDefault();

        const trimmedName = name.trim();
        if (isValidName(trimmedName)) {
            dispatch(join(trimmedName));
        }
    }

    return (
        <>
            <div style={styles.login}>
                <h3>Welcome!</h3>
                <input
                    className="login_input"
                    type="text"
                    name="username"
                    value={name}
                    onChange={handleNameChange}
                />
                <button style={styles.joinButton} onClick={handleJoin}>
                    Join!
                </button>
            </div>
        </>
    );
}
