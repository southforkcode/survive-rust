Feature: Status Command
  As a player
  I want to be able to check my status and environment
  So that I know my health, camp inventory, and weather conditions

  Scenario: Checking general status
    Given the game is running
    When I type the command "status"
    Then the output should contain "The sun is high in the sky."
    And the output should contain "In the camp you see:"
    And the output should contain "A sleeping spot"
    And the output should contain "You feel rested and healthy."
    And the game should still be running

  Scenario: Checking weather status
    Given the game is running
    When I type the command "status weather"
    Then the output should contain "The sun is high in the sky. It's quite warm outside."
    And the game should still be running

  Scenario: Checking camp status default
    Given the game is running
    When I type the command "status camp"
    Then the output should contain "In the camp you see:"
    And the output should contain "No firewood"
    And the output should contain "No water"

  Scenario: Checking camp status after gathering
    Given the game is running
    When I type the command "gather wood"
    And I type the command "status camp"
    Then the output should contain "Some firewood"

  Scenario: Checking player status
    Given the game is running
    When I type the command "status player"
    Then the output should contain "You feel rested and healthy."
    And the game should still be running

  Scenario: Checking unknown status
    Given the game is running
    When I type the command "status unknown"
    Then the output should contain "Cannot get status of unknown target."
    And the game should still be running
