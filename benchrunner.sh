set -e

for C in $(seq 1 100)
do
    for W in $(seq 1 100)
    do
	RESULT_FILENAME="benchresults/c${C}_w${W}.csv"
	if [ ! -e $RESULT_FILENAME ]; then
		echo "concurrency: $C, worker: $W"
		cat Rocket.toml.template | sed -e "s/worker = 1/worker = ${W}/g" > Rocket.toml
		ROCKET_ENV=production cargo run --release &
		ROCKET_PID=$!
		sleep 1
		ab -n 1000 -c $C http://localhost:8000/ # warm up
		ab -n 2000 -c $C -e $RESULT_FILENAME http://localhost:8000/
		kill $ROCKET_PID
		sleep 1
	fi
    done
done

