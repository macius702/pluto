set -e
dfx deploy
echo
curl -X GET -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/pobierz_wpisy"
echo
curl -X POST -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/dodaj_wpis/Ula"
echo
curl -X GET -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/pobierz_wpisy"
echo