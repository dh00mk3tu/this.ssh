export type Status = 'success' | 'failed' | 'loading' | 'idle' 

export interface State {
	status: Status,
    message?: String,
    data?: Object
}

export interface TriggerKeyMenuAnimationIndex  {
    copy: boolean,
    remove: boolean,
}

export interface Key {
    keyPid: string,
    publicKey: string,
    email: string,
    isActive: boolean,
}