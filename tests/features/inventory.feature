Feature: Inventory Command
  As a player
  I want to check my inventory
  So that I know what resources I have gathered

  Scenario: Checking empty inventory
    Given the game is running
    When I type the command "inventory"
    Then the output should contain "~ 0 lb. of firewood"
    And the output should contain "~ 0 L of water"
    And the output should contain "~ 0 lb. of food"
    And the game should still be running

  Scenario: Checking inventory with multiple resources
    Given the game is running
    When I type the command "gather wood"
    And I type the command "gather water"
    And I type the command "gather food"
    And I type the command "inventory"
    Then the output should contain "lb. of firewood"
    And the output should contain "L of water"
    And the output should contain "lb. of food"
    And the game should still be running

  Scenario: Using inventory shorthand command
    Given the game is running
    When I type the command "inv"
    Then the output should contain "~ 0 lb. of firewood"
    And the output should contain "~ 0 L of water"
    And the output should contain "~ 0 lb. of food"
    And the game should still be running

