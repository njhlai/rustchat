"use client";

import { ChangeEvent, FormEvent, useState } from "react";

import { join } from "../api/user/actions";
import { useAppDispatch } from "../app/hooks";

import styles from "../styles/layout.module.css";

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
        <div className={styles.login}>
            <h3>Welcome!</h3>
            <span className={styles.login_form}>
                <input
                    className={styles.login_input}
                    type="text"
                    placeholder="username"
                    value={name}
                    onChange={handleNameChange}
                />
                <button onClick={handleJoin}>Join!</button>
            </span>
        </div>
    );
}
