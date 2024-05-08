#! /usr/bin/env bash
set -e
dfx deploy
echo


dfx canister logs $(dfx canister id http_server) &

# echo Checking wpisy with GET command...
# curl -X GET -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/pobierz_wpisy"
# echo;echo

echo Changing wpisy with POST command...
curl -X POST -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/dodaj_wpis/Ula"
echo;echo

sleep 5

echo Checking wpisy again with GET command...
curl -X GET -H "Content-Type: application/json" -d "{ \"hello\": \"world\" }" "http://$(dfx canister id http_server).localhost:$(dfx info webserver-port)/pobierz_wpisy"
echo;echo