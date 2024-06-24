# 《Rust程序设计》课程实践报告

（个人信息）

小组成员：徐陈皓，吴悦天，方嘉聪，孙嘉伟，胡宇阳

**说明：本报告主要介绍我们的项目`Thunder`的设计和开发过程，因此由所有小组成员合作撰写，每个小组成员都参与了项目的设计和开发，我们每个人上交的报告内容除了照片以外是基本一致的。**

## 我们完成的项目`Thunder`

**GitHub仓库地址：[Thunder-in-Rust](https://github.com/bucket-xv/Thunder-in-Rust)**

**Web版本游戏地址（加载较慢，请耐心等候）：[Thunder](https://bucket-xv.github.io/Thunder-in-Rust/)**

### 概览

我们用`Rust`重制了雷电战机这个经典游戏。我们的游戏基于`Bevy`游戏引擎开发，支持`Web`和`本地开发`两种版本。概要地说，我们为这个项目做了以下工作：

- 我们阅读了`Bevy`游戏引擎的相关文档，学习了其中基本的接口使用方法和`ECS`(Entity, Compenent, System)游戏开发逻辑。
- 我们用2个月的时间完成了游戏的主体设计和开发，包括游戏的基本逻辑、图形界面、音效等。
- 我们写了较为完备的注释并计划生成`doc`文件发布到网上以供潜在的合作者查阅，我们在github仓库上也写了`README.md`并提供`MIT License`以开源项目。
- 我们利用`Bevy`引擎的支持，将游戏部署到了`GitHub Pages`上，使得用户可以直接通过浏览器访问游戏。

### `Bevy`游戏引擎

（胡宇阳）

### 代码设计

#### 页面切换逻辑

（徐陈皓）

#### 主菜单

Thunder的菜单逻辑以一个 Bevy Plugin 的形式实现，这个 plugin 向 App 中添加了若干system，用来控制进入特定状态时各菜单的渲染、点击按钮时的交互以及状态转移前屏幕元素的清除，主要代码如下：

```rust
pub fn menu_plugin(app: &mut App) {
    app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        // Systems to handle the level picking menu screen
        .add_systems(OnEnter(MenuState::Levels), level_select_menu_setup)
        .add_systems(Update, (level_button.run_if(in_state(MenuState::Levels)),))
        .add_systems(
            OnExit(MenuState::Levels),
            despawn_screen::<OnLevelsMenuScreen>,
        )
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        // Systems to handle the display settings screen
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            display_settings_menu_setup,
        )
        .add_systems(
            Update,
            (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)),),
        )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenuScreen>,
        )
        // Systems to handle the sound settings screen
        .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
        .add_systems(
            Update,
            setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(
            OnExit(MenuState::SettingsSound),
            despawn_screen::<OnSoundSettingsMenuScreen>,
        )
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnEnter(MenuState::Help), help_screen_setup)
        .add_systems(OnExit(MenuState::Help), despawn_screen::<OnHelpScreen>);
}
```

具体来说，菜单是一个 State Machine, `menu.rs` 中定义了一个 `MenuState`: 


```rust 
pub enum MenuState {
    Main,
    Levels,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Help,
    #[default]
    Disabled,
}
```

从这个 enum 的定义可以清晰地看出菜单的几个状态，即主菜单，关卡选择，设置，设置显示，设置声音，帮助和禁用状态。主菜单样式如图：

![image-20240623143346139](./report.assets/image-20240623143346139.png)

System `main_menu_setup` 会在 `Menustate` 变为 `Main` 状态时被 schedule，利用 `commands.spawn` 和一个 `asset_server` (用来加载图标等多媒体素材)，在屏幕中央渲染出菜单界面以及其中的各个按钮。菜单界面的层次结构是利用 spawn bundle 时的 parent-children 特性实现，在 spawn 一个 bundle 后，可以用 `.with_children(|parent| {...})` 传入一个closure，closure中可以继续用`parent.spawn` 生成子元素。例如上图中，最底层的背景板是一个 `NodeBundle`, 它的儿子是另一个`NodeBundle`即中间的红框，红框有五个儿子，分别是一个 `TextBundle` "Thunder" 和四个 `ButtonBundle`, 而四个 `ButtonBundle` 分别各有一个儿子 `TextBundle` 用来写对应的图标和文字。这种树形结构使得菜单的组织非常清晰。由此就可以写出如下的代码：

```rust
pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Thunder",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - help
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::SelectLevel,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "New Game",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/wrench.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::GoToHelp,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/help.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Help", button_text_style.clone()));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section("Exit", button_text_style));
                        });
                });
        });
}
```

Button 被点击之后的跳转也是通过若干个 system 实现，他们监听按钮交互事件并对`MenuState` 和 `GameState` 作出相应的修改，从而使游戏在 State Machine 上移动。例如 `menu_action` 是对主菜单各按钮点击交互的处理：

```rust
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::SelectLevel => {
                    menu_state.set(MenuState::Levels);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::GoToHelp => {
                    menu_state.set(MenuState::Help);
                }
            }
        }
    }
}

```

由于菜单部分细节较多，实现也较为繁琐（800+ 行rust），报告中只介绍这些核心实现思想和方法，更加具体的实现请参考源代码及其中的注释。

#### 关卡选择

在主菜单中点击 New Game 会进入关卡选择界面。

![image-20240623154431320](./report.assets/image-20240623154431320.png)

关卡的实现机制是定义了一个全局的 resource：

```rust
// Levels to play that can be choose in the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Level(u32);
```

在菜单中用上文介绍的方法和思路渲染出 level select menu，并设计一个 level button 的交互系统。与设置部分等按钮交互系统不同的是，level按钮在选择之后，应该既像其他设置一样更新对应的resource，但又不能仍然停留在 menu 界面，而是应该进入游戏。具体实现中，我们还加入了一个 level splash screen（`level_splash.rs`) ，用于在进入游戏之前显示当前关卡，level splash screen 结束之后进入游戏。在生成敌人以及配置武器时，会根据 level 这个 resource 的值来进行不同的配置，从而实现关卡机制。

#### 结算

结算界面设计为`gmae`下的一个子module`win_lose_screen`，在其中实现了胜利界面、失败界面和通关界面，同样使用Bevy Plugin的形式实现，三个界面的实现逻辑类似，下面以胜利界面为例：

```rust
pub fn win_lose_screen_plugin(app: &mut App) {
    app.init_state::<WinLoseScreenState>()
        .add_systems(OnEnter(GameState::Win), win_screen_setup)
        .add_systems(OnEnter(GameState::Lose), lose_screen_setup)
        .add_systems(OnEnter(GameState::Completion), completion_screen_setup)
        .add_systems(
            OnEnter(WinLoseScreenState::BackToMainMenu),
            back_to_main_menu,
        )
        .add_systems(OnEnter(WinLoseScreenState::Restart), restart_level)
        .add_systems(OnEnter(WinLoseScreenState::NextLevel), next_level)
        .add_systems(
            Update,
            (win_lose_screen_action, button_system).run_if(in_state(GameState::Win)),
        )
        .add_systems(
            Update,
            (win_lose_screen_action, button_system).run_if(in_state(GameState::Lose)),
        )
        .add_systems(
            Update,
            (win_lose_screen_action, button_system).run_if(in_state(GameState::Completion)),
        )
        .add_systems(OnExit(GameState::Win), despawn_screen::<OnWinScreen>)
        .add_systems(OnExit(GameState::Lose), despawn_screen::<OnLoseScreen>)
        .add_systems(
            OnExit(GameState::Completion),
            despawn_screen::<OnCompleteScreen>,
        );
}
```

胜利界面的`enum`如下：

```rust
enum WinLoseScreenState {
    BackToMainMenu,
    Restart,
    NextLevel,
    #[default]
    Disabled,
}
```

![Win](./report.assets/Win.png)

通过监控游戏中的Player血量以及剩余的敌人数量来判定游戏的输赢(`GameState::Win`)，`Retry`按钮进入`WinLoseScreenState::Restart`，`NextLevel`会进入`WinLoseScreenState::NextLevel`，进而调用system`next_level`更新关卡参数`level_setting`进入下一关(最后一关进行特判确定是否通过所有关卡`GameState::Completion`)。

#### 暂停

Thunder的的暂停菜单设计为`game`下的一个子module`esc_menu`，以一个Bevy Plugin的形式实现，和主菜单逻辑类似，通过进出状态(State-System)控制各菜单的渲染、点击按钮时的交互以及状态转移前屏幕元素的清除，具体的代码如下： 

```rust
pub fn esc_menu_plugin(app: &mut App) {
    app.init_state::<EscMenuState>()
        .add_systems(OnEnter(GameState::Stopped), esc_menu_setup)
        .add_systems(OnEnter(EscMenuState::MainEscMenu), esc_main_menu_setup)
        .add_systems(OnEnter(EscMenuState::BackToMainMenu), back_to_main_menu)
        .add_systems(OnEnter(EscMenuState::BackToGame), back_to_game)
        .add_systems(OnEnter(EscMenuState::Help), help_screen_setup)
        .add_systems(OnExit(EscMenuState::Help), despawn_screen::<OnHelpScreen>)
        .add_systems(
            Update,
            (esc_menu_action, button_system).run_if(in_state(EscMenuState::Help)),
        )
        .add_systems(
            Update,
            (esc_menu_action, button_system).run_if(in_state(EscMenuState::MainEscMenu)),
        )
        .add_systems(
            OnExit(EscMenuState::MainEscMenu),
            despawn_screen::<OnMainEscMenuScreen>,
        );
}
```

更具体地，我们将暂停菜单的状态定义为一个`enum`：

```rust
enum EscMenuState {
    MainEscMenu, // The main menu screen
    BackToGame,  // The screen that appears when the player clicks the "Back to Game" button
    BackToMainMenu,
    Help,
    #[default]
    Disabled,
}
```

通过在游戏主界面`game`实现一个Menu按钮，点按该按钮会进入暂停菜单(切换到`GameState::Stopped`状态)。同时为了方便用户操作，我们通过监听键盘事件，当用户按下`Esc`键时，也会进入暂停菜单。 

在暂停菜单中，我们提供了四个选项：返回游戏`Continue`，返回主菜单`Home`，帮助`Help`和退出游戏`Exit`。具体界面如下：

![Pause](./report.assets/Pause.png)

和主界面逻辑类似，在进入`GameState::Stopped`后会调用`esc_menu_setup`将`EscMenuState`设置成`MainEscMenu`，进而调用`esc_main_menu_setup`。使用`commands.spawn` 和`asset_server` (用来加载图标等多媒体素材)，在屏幕中央渲染出菜单界面以及其中的各个按钮。层次结构是利用 `spawn bundle` 时的 `parent-children` 特性实现。详细代码较为复杂，不在报告中展示，可以详参源码。

Button 被点击之后的跳转也是通过若干个 system 实现，监听按钮交互事件并对`EscMenuState` 和 `EscGameState`和`GameState` 作出相应的修改，由此相应的游戏跳转逻辑，详细代码见下:

```rust
fn esc_menu_action(
    interaction_query: Query<
        (&Interaction, &EscMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut esc_menu_state: ResMut<NextState<EscMenuState>>,
    // mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, esc_menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match esc_menu_button_action {
                EscMenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                EscMenuButtonAction::BackToMainMenu => {
                    esc_menu_state.set(EscMenuState::BackToMainMenu);
                }
                EscMenuButtonAction::BackToGame => {
                    esc_menu_state.set(EscMenuState::BackToGame);
                }
                EscMenuButtonAction::GoToHelp => {
                    esc_menu_state.set(EscMenuState::Help);
                }
                EscMenuButtonAction::BackToEscMenu => {
                    esc_menu_state.set(EscMenuState::MainEscMenu);
                }
            }
        }
    }
}
```

#### 游戏逻辑

（吴悦天&徐陈皓）

![image-20240623155022954](./report.assets/image-20240623155022954.png)

Thunder 的游戏逻辑在 `game` 模块中实现，`game.rs` 实现了一个插件 `game_plugin`：

```rust
// This plugin will contain the game. It will focus on the state `GameState::Game`
pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Init), game_setup)
        .add_event::<HittingEvent>()
        .add_systems(OnEnter(GameState::Game), setup_laser)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                generate_enemy,
                shoot_gun,
                apply_velocity,
                clear_laser,
                move_player_plane,
                shoot_laser,
                check_for_bullet_hitting,
                check_for_laserray_hitting,
                check_for_laser_star_capture,
                play_hitting_sound,
                update_scoreboard,
                update_hpboard,
                update_laserboard,
                check_for_next_wave,
                add_laser_star,
                remove_laser_star,
            )
                // `chain`ing systems together runs them in order
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (button_system, game_menu_action, back_on_esc).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnEnter(GameState::Menu),
            (despawn_screen::<OnGameScreen>, restore_background),
        )
        .add_systems(
            OnEnter(GameState::Win),
            (despawn_screen::<OnGameScreen>, restore_background),
        )
        .add_systems(
            OnEnter(GameState::Lose),
            (despawn_screen::<OnGameScreen>, restore_background),
        )
        .add_systems(
            OnEnter(GameState::Completion),
            (despawn_screen::<OnGameScreen>, restore_background),
        );
}
```

其中实现核心逻辑的是每一帧更新时执行的一系列 systems……

另一种武器是激光 laser，其对应的“子弹”为”镭射“ laser ray。和普通子弹不同的是，激光镭射是触发时立刻打出一个矩形向上方射出，该矩形延伸至上方边界，并会对矩形内部所有敌机产生伤害。

### 网页端的开发与部署



游戏的网页端使用`GitHub Pages`部署，我们在`GitHub`仓库的`Settings`中找到`GitHub Pages`选项，将`Source`设置为`master`分支中的，这样就可以通过`https://bucket-xv.github.io/Thunder-in-Rust/`访问我们的游戏。

### 项目开发过程

### 项目展望

## 个人照片

（照片）
（学期感言？）
