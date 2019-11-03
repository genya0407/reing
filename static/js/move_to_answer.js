const app = new Vue({
    el: '#app',
    data: {
        answer: {
            created_at_recognizable: '',
            body: '',
            question: {
                body: ''
            }
        },
        next_answer: null, // same structure as answer
        prev_answer: null  // same structure as answer
    },
    mounted: function () {
        this.$nextTick()
            .then(() => {
                const answer_id = location.pathname.match(/\/answer\/(\d+)/)[1]
                return fetch(`/api/answer/${Number(answer_id)}`)
            })
            .then((response) => response.json())
            .then(this.updatePropsByData)
    },
    methods: {
        intent_url: function () {
            return `https://twitter.com/intent/tweet?url=${encodeURIComponent(window.location)}&text=${encodeURIComponent(this.answer.body)}&hashtags=reing`
        },
        moveToNext: function () {
            if (!this.next_answer) {
                return
            }

            this.answer = this.next_answer
            fetch(`/api/answer/${Number(this.next_answer.id)}`)
                .then((response) => response.json())
                .then(this.updatePropsByData)
        },
        moveToPrev: function () {
            if (!this.prev_answer) {
                return
            }

            this.answer = this.prev_answer
            fetch(`/api/answer/${Number(this.prev_answer.id)}`)
                .then((response) => response.json())
                .then(this.updatePropsByData)
        },
        moveToRandom: function () {
            fetch(`/api/answer/random`)
                .then((response) => response.json())
                .then(this.updatePropsByData)
        },
        updatePropsByData: function (data) {
            this.answer = {
                id: data.answer.id,
                created_at_recognizable: data.answer.created_at_recognizable,
                body: data.answer.body,
                question: {
                    body: data.answer.question.body
                }
            }
            window.history.replaceState(null, null, `/answer/${Number(data.answer.id)}`)

            if (data.next_answer) {
                this.next_answer = {
                    id: data.next_answer.id,
                    created_at_recognizable: data.next_answer.created_at_recognizable,
                    body: data.next_answer.body,
                    question: {
                        body: data.next_answer.question.body
                    }
                }
            } else {
                this.next_answer = null
            }

            if (data.prev_answer) {
                this.prev_answer = {
                    id: data.prev_answer.id,
                    created_at_recognizable: data.prev_answer.created_at_recognizable,
                    body: data.prev_answer.body,
                    question: {
                        body: data.prev_answer.question.body
                    }
                }
            } else {
                this.prev_answer = null
            }
        }
    }
})