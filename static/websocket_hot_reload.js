const socket = new WebSocket(`ws://${window.location.host}/ws/`)

socket.onopen = (event) => {
	console.log('Hot reload WebSocket opened')

	if (sessionStorage.getItem('socket') === null || sessionStorage.getItem('socket') === 'false') {
		location.reload()
	}

	sessionStorage.setItem('socket', true)
}

socket.onclose = (event) => {
	console.log('Waiting on the server to restart')
	sessionStorage.setItem('socket', false)

	let interval = setInterval(() => {
		fetch('/api/v1/health/alive')
			.then((response) => {
				if (response.ok) {
					// You can handle the successful response here
					console.log('Successfully called the endpoint')
					clearInterval(interval)
					location.reload()
				}
			})
			.catch(() => {})
	}, 100)
}
