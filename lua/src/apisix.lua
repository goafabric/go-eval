local http = require("resty.http")
local cjson = require("cjson.safe")
local url = "http://callee-service-application.example:8080/authorizations/getTenantBySubject"

local httpc = http.new()
httpc:set_timeout(100)
local res, err = httpc:request_uri(url, {
    method = "GET", headers = ngx.req.get_headers(),
})

if not res or res.status ~= 200 then
    ngx.log(ngx.ERR, "Tenant Resolution failed: ", err or ("HTTP status " .. (res and res.status or "unknown")))
    ngx.say("Tenant Resolution failed")
    ngx.exit(ngx.HTTP_BAD_REQUEST)
end

local tenantId = res.body
ngx.req.set_header("X-TenantId", tenantId)
ngx.log(ngx.ERR, "X-TenantId set to: ", tenantId)
ngx.log(ngx.ERR, "userinfo is ", ngx.req.get_headers()["x-userinfo"])

-- exemplary section if we would want to use the token endpoint, that should return token + tenantId
-- local data, decode_err = cjson.decode(res.body)
-- ngx.req.set_header("Authentication", "Bearer " .. data.token)