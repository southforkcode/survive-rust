Feature: Gather Command Events
  As a player
  I want random events to occur during gathering
  So that the game is more dynamic and challenging

  Scenario: Abandoned Stash event increases resources
    Given the game is running
    When an AbandonedStash event is executed
    Then either food or water inventory should increase

  Scenario: Hive Attack event damages player
    Given the game is running
    When a HiveAttack event is executed
    Then the player's health should decrease
