Feature: Basic Game Loop
  As a player
  I want to be able to start the game and type commands
  So that I can interact with the game engine

  Scenario: Processing help command
    Given the game is running
    When I type the command "help"
    Then the output should contain "Available commands"
    And the game should still be running

  Scenario: Processing quit command
    Given the game is running
    When I type the command "quit"
    Then the output should contain "Exiting game"
    And the game should not be running
