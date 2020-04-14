import {useEffect, useState} from 'react'

export function useDebouncedState<T>(state: T, timeout: number = 500): T {
    let [debouncedState, setDebouncedState] = useState(state)

    useEffect(() => {
        let timer = setTimeout(() => setDebouncedState(state), timeout)

        return () => clearTimeout(timer)
    }, [state, timeout])

    return debouncedState
}