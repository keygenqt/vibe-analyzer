/**
 * Module documentation for ArkTS testing
 */

import { AbilityConstant, UIAbility, Want } from '@kit.AbilityKit';
import { window } from '@kit.ArkUI';

/**
 * Entry point class
 */
@Entry
@Component
struct Index {
  @State message: string = 'Hello World';

  /**
   * Build method
   */
  build() {
    Column() {
      Text(this.message)
        .fontSize(50)
        .fontWeight(FontWeight.Bold)
    }
    .width('100%')
    .height('100%')
  }
}

/**
 * User data interface
 */
export interface User {
  name: string;
  age: number;
}

/**
 * Color enum
 */
export enum Color {
  Red,
  Green,
  Blue
}

/**
 * Service class
 */
export class UserService {
  private users: User[] = [];

  /**
   * Add user method
   */
  addUser(user: User): void {
    this.users.push(user);
  }

  /**
   * Get users method
   */
  getUsers(): User[] {
    return this.users;
  }
}

/**
 * Async function example
 */
export async function fetchData(url: string): Promise<string> {
  return "data";
}

/**
 * Global constant
 */
export const MAX_SIZE: number = 1024;

/**
 * Global variable
 */
export let appName: string = "vibe-analyzer";
