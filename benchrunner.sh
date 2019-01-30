set -e

for C in $(seq 1 100)
do
    for W in $(seq 1 100)
    do
        echo "concurrency: $C, worker: $W"
        cat Rocket.toml.template | sed -e "s/worker = 1/worker = ${W}/g" > Rocket.toml
        ROCKET_ENV=production cargo run --release &
        ROCKET_PID=$!
        sleep 1
        ab -n 1000 -c $C http://localhost:8000/ # warm up
        ab -n 2000 -c $C -e "benchresults/c${C}_w${W}.csv" http://localhost:8000/
        kill $ROCKET_PID
        sleep 1
    done
done

