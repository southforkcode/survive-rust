Feature: Gather command
	As a player
	I want to gather resources
	So that I can survive longer

	Scenario: Gather wood
		Given the game is running
		When I type the command "gather wood"
		Then the output should contain "Gathered 100 lbs. of wood!"
		And the game should still be running

	Scenario: Gather water
		Given the game is running
		When I type the command "gather water"
		Then the output should contain "Gathered 100 liters of water!"
		And the game should still be running

	Scenario: Gather food
		Given the game is running
		When I type the command "gather food"
		Then the output should contain "Gathered 100 lbs. of food!"
		And the game should still be running

	Scenario: Gather without resource name
		Given the game is running
		When I type the command "gather"
		Then the output should contain "Couldn't gather unknown resource!"
		And the game should still be running

	Scenario: Gather unsupported resource
		Given the game is running
		When I type the command "gather stone"
		Then the output should contain "Couldn't gather unknown resource!"
		And the game should still be running
