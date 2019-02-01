set -e
echo "concurrency,workers,longest_request(ms)" > $1

for C in $(seq 1 12)
do
    for W in $(seq 1 12)
    do
        echo "concurrency: $C, worker: $W"
        cat Rocket.toml.template | sed -e "s/worker = 1/worker = ${W}/g" > Rocket.toml
        ROCKET_ENV=production cargo run --release &
        ROCKET_PID=$!
        sleep 1
        ab -n 1000 -c $C http://localhost:8000/ # warm up
        LONGEST_REQUEST=$(ab -n 2000 -c $C http://localhost:8000/ 2> /dev/null | grep "longest request" | awk '{ print $2 }')
        echo "$C,$W,$LONGEST_REQUEST" >> $1
        kill $ROCKET_PID
        sleep 1
    done
done

