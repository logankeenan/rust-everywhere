#!/bin/bash

BASE_URL="http://localhost:3000"
USER_ID="123e4567-e89b-12d3-a456-426614174000"
OTHER_USER_ID="123e4567-e89b-12d3-a456-426614174001"
TEST_NOTE='{"content": "This is a test note"}'
UPDATED_NOTE='{"content": "This note has been updated"}'

# Function to extract HTTP status code
extract_status_code() {
    echo $1 | grep HTTP |  awk '{print $2}'
}

# Function to assert HTTP status code in the 200 range
assert_status_code() {
    local status_code=$1
    if [ $status_code -lt 200 ] || [ $status_code -gt 299 ]; then
        echo "Expected a status code in the range 200-299, but got $status_code"
        # Exit the script if assertion fails
        kill -9 $SERVER_PID
        exit 1
    fi
}

# Function to count the number of notes in a GET /notes response
count_notes() {
    echo $1 | grep -o '"id"' | wc -l
}

. ./sqlx.sh database drop -y
. ./sqlx.sh database create
. ./sqlx.sh migrate run

# Start the server in the background in its own session
nohup cargo run > /dev/null 2>&1 &
# Save the PID of the server process
SERVER_PID=$!

# Allow server to start
sleep 5

echo "Testing POST /notes"
response=$(curl -H "user-id: $USER_ID" -H "Content-Type: application/json" -X POST -d "$TEST_NOTE" -v "$BASE_URL/notes" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
note_id=$(echo $response | jq -r '.id')
echo

echo "Testing GET /notes/:id"
response=$(curl -H "user_id: $USER_ID" -v "$BASE_URL/notes/$note_id" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
echo

echo "Testing PATCH /notes/:id"
response=$(curl -H "user_id: $USER_ID" -H "Content-Type: application/json" -X PATCH -d "$UPDATED_NOTE" -v "$BASE_URL/notes/$note_id" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
echo

echo "Testing POST /notes with different user id"
response=$(curl -H "user-id: $OTHER_USER_ID" -H "Content-Type: application/json" -X POST -d "$TEST_NOTE" -v "$BASE_URL/notes" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
echo

echo "Testing GET /notes"
response=$(curl -H "user-id: $USER_ID" -v "$BASE_URL/notes" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
# Check the number of notes is as expected
num_notes=$(count_notes "$response")
if [ $num_notes -ne 1 ]; then
    echo "Expected 1 note, but got $num_notes"
    kill -9 $SERVER_PID
    exit 1
fi
echo

echo "Testing POST /notes for searchable note"
response=$(curl -H "user-id: $USER_ID" -H "Content-Type: application/json" -X POST -d "$SEARCH_NOTE" -v "$BASE_URL/notes" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
search_note_id=$(echo $response | jq -r '.id')
echo

echo "Testing GET /notes?q=should"
response=$(curl -H "user-id: $USER_ID" -G -d "q=should" -v "$BASE_URL/notes" 2>&1)
status_code=$(extract_status_code "$response")
assert_status_code $status_code
# Check the number of notes is as expected
num_notes=$(count_notes "$response")
if [ $num_notes -ne 1 ]; then
    echo "Expected 1 note, but got $num_notes"
    kill -9 $SERVER_PID
    exit 1
fi
echo

kill -9 $SERVER_PID