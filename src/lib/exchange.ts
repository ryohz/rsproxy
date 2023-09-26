import { capitalize } from "./common";

export class Request {
    public headers: string;
    public version: string;
    public method: string;
    public url: string;
    public body: string;
    public pair_id: string;
    public is_empty: boolean;

    constructor(args: RustRequest, empty?: boolean) {
        this.headers = args.headers;
        this.version = args.version;
        this.method = args.method;
        this.url = args.url;
        this.body = args.body;
        this.to_editable();

        if (empty !== undefined) {
            this.is_empty = empty;
        } else {
            this.is_empty = false;
        }

        let headers: Record<string, string> = JSON.parse(this.headers);
        this.pair_id = headers["pair-id"];
    }

    public to_editable(): string {
        let rq = "";
        let headers: Record<string, string> = JSON.parse(this.headers);
        // method path version
        let host = headers['host'];
        let path = "/"
        console.log(this.url);
        if (host !== undefined) {
            path = this.url.split(host)[1];
        }
        rq += `${this.method.toUpperCase()} ${path} ${this.version}\n`;

        // headers
        rq += headers_to_editable(headers);

        // body
        rq += "\n"
        rq += this.body;

        return rq;
    }

}

// 0 "GET /index.html HTTP/1.1"
// 1 "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=…"
// 2 "Accept-Encoding: gzip, deflate"
// 3 "Accept-Language: en-US,en;q=0.9"
// 4 "Cache-Control: max-age=0"
// 5 "Host: www.chiseki.go.jp"
// 6 "Proxy-Connection: keep-alive"
// 7 ": 1"
// 8 "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36"
// 9 ""
// 10 ""
export interface EditableRequest {
    url: string,
    content: string,
}

export function to_request(req_str: string) {
    let req_str_list = req_str.split("\n");
    let parts = req_str_list[0].split(" ");
    let headers: Record<string, string> = {};
    let body = ""
    let now = "h"
    let first_empty_flag = false;
    for (let i = 1; i < req_str_list.length - 1; i++) {
        let item = req_str_list[i]
        console.log(item);
        if (item === "" && !first_empty_flag) {
            first_empty_flag = true;
            continue;
        }
        if (!first_empty_flag) {
            let h_str_list = item.split(" ");
            let name_part_list = h_str_list[0].split("-");
            let name_list: string[] = []
            name_part_list.forEach(item => {
                name_list.push(item.toLowerCase())
            });
            let name = name_list.join("-");

            let value = h_str_list[1]
            headers[name] = value;
        } else {
            body += item + "\n";
        }
    }

    let method = parts[0];

}

export function empty_request(): Request {
    return new Request({
        headers: "{}",
        version: "",
        method: "",
        url: "",
        body: "empty request",
    }, true);
}

export class Response {
    public headers: string;
    public version: string;
    public status: string;
    public body: string;
    public pair_id: string;
    public is_empty: boolean;

    constructor(args: RustResponse, empty?: boolean) {
        this.headers = args.headers;
        this.version = args.version;
        this.status = args.status.toString();
        this.body = args.body;

        if (empty !== undefined) {
            this.is_empty = empty;
        } else {
            this.is_empty = false;
        }


        let headers: Record<string, string> = JSON.parse(this.headers);
        this.pair_id = headers["pair-id"];
    }

    public to_editable(): string {
        let rs = "";
        let headers: Record<string, string> = JSON.parse(this.headers);

        // version, status
        rs += `${this.version} ${this.status}\n`;

        // headers
        rs += headers_to_editable(headers);

        //body
        rs += "\n"
        rs += this.body;

        return rs;
    }
}

export function empty_response(): Response {
    return new Response({
        headers: "{}",
        status: 0,
        version: "",
        body: "empty response",
    }, true);
}

function headers_to_editable(headers: Record<string, string>): string {
    let h = "";
    for (const key in headers) {
        let parts = key.split("-");
        let upper_parts: string[] = []
        parts.forEach(part => {
            upper_parts.push(capitalize(part));
        });
        let header_name = upper_parts.join("-");
        if (header_name !== "Pair-Id") {
            h += `${header_name}: ${headers[key]}\n`;
        }
    }
    return h;
}

export interface RustRequest {
    headers: string;
    version: string;
    method: string;
    url: string;
    body: string;
}

export interface RustResponse {
    headers: string;
    version: string;
    status: number;
    body: string;
}