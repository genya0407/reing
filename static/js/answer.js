(function(){
    function answerChanged(text) {
        document.getElementById('current-character-count').textContent = text.length;

        const submit_button = document.getElementById('submit-button');
        if (text.length > 110) {
            submit_button.disabled = true;
        } else {
            submit_button.disabled = false;
        }
    }

    document.getElementById('answer-textarea').addEventListener('keyup', function(event){
        console.log(event);
        answerChanged(event.target.value)
    });
})()