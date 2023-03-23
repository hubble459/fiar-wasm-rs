export declare interface instanceObject {
    [key: string]: string;
}

/**
 * JSON转url参数
 * @param data Json格式数据
 * */
export function formatJsonToUrlParams(data: instanceObject) {
    return (typeof data === 'object'
        ? Object.keys(data)
            .map((key) => `${encodeURIComponent(key)}=${encodeURIComponent(data[key])}`)
            .join('&')
        : '');
}

export default formatJsonToUrlParams;
