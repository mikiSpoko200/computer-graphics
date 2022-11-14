// ==========================================================================
// AGL3:  GL/GLFW init AGLWindow and AGLDrawable class definitions
//
// Ver.3  14.I.2020 (c) A. Łukaszewski
// ==========================================================================
// AGL3 example usage 
//===========================================================================
#include <stdlib.h>
#include <stdio.h>
#include <random>

#define _USE_MATH_DEFINES
#include <math.h>


#include <glm/vec2.hpp>
#include <glm/mat2x2.hpp>

#include "components.hpp"
#include "helpers.h"
#include "types.h"
#include "AGL3Window.hpp"


namespace _2d {
    enum class Rotation {
        Clockwise,
        Counterclockwise,
    };

    enum class Direction {
        Up,
        Down,
        Left,
        Right,
    };


    struct IRotatable {
        virtual void rotate(GLfloat angle) = 0;
    };

    struct IMovable {
        virtual void move(glm::vec2 displacement) = 0;
    };

    struct IFixedRotatable {
        virtual void rotate(Rotation direction) = 0;

        virtual void set_angular_velocity(f32 angular_velocity) = 0;

        virtual f32 get_angular_velocity() const = 0;
    };

    struct IFixedMovable {
        virtual void move(Direction direction) = 0;

        virtual void set_velocity(glm::vec2 velocity) = 0;

        virtual glm::vec2 get_velocity() const = 0;
    };


    struct Segment {
        glm::vec2 p1;
        glm::vec2 p2;

        void move(const glm::vec2 displacement) {
            this->p1 += displacement;
            this->p2 += displacement;
        }

        void rotate(const GLfloat angle) {
            const glm::vec2 initial_position = this->center();
            const glm::vec2 translation = initial_position * -1.0f;

            const glm::mat2 rotation = {
                {std::cos(angle), -std::sin(angle)},
                {std::sin(angle), std::cos(angle)}
            };

            this->move(translation);
            this->p1 = rotation * this->p1;
            this->p2 = rotation * this->p2;
            this->move(initial_position);
        }

        glm::vec2 center() const {
            glm::vec2 center = p1 + p2;
            center /= 2.0;
            return center;
        }

        f32 angle() const {
            glm::vec2 centered_p1 = this->p1 - this->center();

            return std::atan2(centered_p1.y, centered_p1.x);
        }

        f32 constantCoefficient() const {
            return p1.y - p1.x * linearCoefficient();
        }

        f32 linearCoefficient() const {
            return (p2.y - p1.y) / (p2.x - p1.x);
        }

        glm::vec2 leftmost() const {
            return p1.x <= p2.x ? p1 : p2;
        }

        glm::vec2 rightmost() const {
            return p1.x > p2.x ? p1 : p2;
        }
    };

    struct KinematicSegment : IFixedMovable, IFixedRotatable {
        Segment base;
        glm::vec2 velocity;
        f32 angular_velocity;

        void move(Direction direction) override {
            glm::vec2 displacement = { 0.0, 0.0 };
            switch (direction) {
            case Direction::Up: {
                displacement.y += velocity.y;
            } break;
            case Direction::Down: {
                displacement.y -= velocity.y;
            } break;
            case Direction::Right: {
                displacement.x += velocity.x;
            } break;
            case Direction::Left: {
                displacement.x -= velocity.x;
            } break;
            }

            this->base.p1 += displacement;
            this->base.p2 += displacement;
        }

        void set_velocity(glm::vec2 velocity) override {
            this->velocity = velocity;
        }

        glm::vec2 get_velocity() const override {
            return this->velocity;
        }

        void rotate(Rotation direction) override {
            GLfloat direction_mult = 1.0f;
            if (Rotation::Counterclockwise == direction) {
                direction_mult = -1.0f;
            }

            this->base.rotate(direction_mult * this->angular_velocity);
        }

        void set_angular_velocity(f32 angular_velocity) override {
            this->angular_velocity = angular_velocity;
        }

        f32 get_angular_velocity() const override {
            return this->angular_velocity;
        }
    };

    struct Grid {
        GLfloat tail_size;
        usize row_tail_count;

        Grid(usize size) : row_tail_count(size), tail_size(2.0f / (GLfloat)size) {
            VDEBUGLN("Creating maze grid");
            VDEBUGFMTLN("-- Tail count: %d", this->row_tail_count);
            VDEBUGFMTLN("-- Tail size : %f", this->tail_size);
        }

        Segment get_sample_segment() const {
            VDEBUGLN("Requesting a sample segment");
            Segment horizontal;
            horizontal.p1 = glm::vec2(-tail_size / 2.0f, 0.0f);
            horizontal.p2 = glm::vec2(tail_size / 2.0f, 0.0f);
            VDEBUGFMTLN("-- (%f, %f), (%f, %f)", horizontal.p1.x, horizontal.p1.y, horizontal.p2.x, horizontal.p2.y);
            return horizontal;
        }

        glm::vec2 player_start_position() const {
            const glm::vec2 offset = { -1.0 + this->tail_size / 2.0, -1.0 + this->tail_size / 2.0 };
            DEBUGLN("Player starting position");
            DEBUGFMTLN("-- (%f, %f)", offset.x, offset.y);
            return offset;
        }

        glm::vec2 get_tail_center(usize column_index, usize row_index) const {
            VDEBUGFMTLN("Column index: %d, Row index: %d", column_index, row_index);
            const glm::vec2 tail_top_left_corner = { (GLfloat)row_index * this->tail_size, (GLfloat)column_index * this->tail_size };
            VDEBUGFMTLN("Tail top left corner: (%f, %f)", tail_top_left_corner.x, tail_top_left_corner.y);
            const glm::vec2 tail_center = tail_top_left_corner + glm::vec2(tail_size / 2.0f, tail_size / 2.0f);
            VDEBUGFMTLN("Tail center: (%f, %f)", tail_center.x, tail_center.y);
            const glm::vec2 scaled_coordinates = tail_center - glm::vec2(1.0f, 1.0f);
            VDEBUGFMTLN("Tail coordinates scaled: (%f, %f)", scaled_coordinates.x, scaled_coordinates.y);
            return scaled_coordinates;
        }
    };

    namespace collisions {
        bool ccw(glm::vec2 a, glm::vec2 b, glm::vec2 c) {
            return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x);
        }

        bool betterDoCollide(const Segment& lhs, const Segment& rhs) {
            return ccw(lhs.p1, rhs.p1, rhs.p2) != ccw(lhs.p2, rhs.p1, rhs.p2) && ccw(lhs.p1, lhs.p2, rhs.p1) != ccw(lhs.p1, lhs.p2, rhs.p2);
        }

        bool doCollide(const Segment& lhs, const Segment& rhs) {
            Segment leftmost, rightmost;
            printf("lhs: (%f, %f), (%f, %f)", lhs.p1.x, lhs.p1.y, lhs.p2.x, lhs.p2.x);
            printf("rhs: (%f, %f), (%f, %f)", rhs.p1.x, rhs.p1.y, rhs.p2.x, rhs.p2.x);
            if (lhs.leftmost().x <= rhs.leftmost().x) {
                leftmost = lhs;
                rightmost = rhs;
            }
            else {
                leftmost = rhs;
                rightmost = lhs;
            }
            printf("leftmost: (%f, %f), (%f, %f)", leftmost.p1.x, leftmost.p1.y, leftmost.p2.x, leftmost.p2.x);
            printf("rightmost: (%f, %f), (%f, %f)", rightmost.p1.x, rightmost.p1.y, rightmost.p2.x, rightmost.p2.x);

            // segments do not overlap on the x axis.
            if (leftmost.rightmost().x < rightmost.leftmost().x) {
                return false;
            }
            // segments are parallel so in order to overlap need to be collinear.
            if (leftmost.linearCoefficient() == rightmost.linearCoefficient()) {
                return rightmost.constantCoefficient() == leftmost.constantCoefficient();
            }

            // calculate the x coordinate of the intersection point.
            float intersectionX = (rightmost.constantCoefficient() - leftmost.constantCoefficient()) /
                (leftmost.linearCoefficient() - rightmost.linearCoefficient());

            return rightmost.leftmost().x <= intersectionX && intersectionX <= leftmost.rightmost().x;
        }
    }

    struct MazeDrawCtx {
        Segment model;
        const std::vector<GLfloat>& rotations;
        std::array<glm::vec3, 2>& colors;
        const Grid& grid;

        MazeDrawCtx(const std::vector<GLfloat>& rotations, std::array<glm::vec3, 2>& colors, const Grid& grid) :
            grid(grid),
            rotations(rotations),
            colors(colors)
        {
            this->model = grid.get_sample_segment();
        }
    };

    struct Maze : public components::IDrawable<3> {
        static const GLuint BUFFER_ID_MODEL = 0;
        static const GLuint BUFFER_ID_COLOR = 1;
        static const GLuint BUFFER_ID_ROTATION = 2;

        static const GLuint SHADER_ATTRIBUTE_ID_MODEL = 0;
        static const GLuint SHADER_ATTRIBUTE_ID_COLOR = 1;
        static const GLuint SHADER_ATTRIBUTE_ID_ROTATION = 2;

        const Grid grid;
        std::vector<Segment> segments;
        std::vector<GLfloat> rotations;
        std::array<glm::vec3, 2> colors;
        components::StateManager<3> gl_manager;
        MazeDrawCtx draw_ctx;

        bool check_collisions(const Segment& player) const {
            for (const auto& segment : this->segments) {
                if (collisions::betterDoCollide(player, segment)) {
                    return true;
                }
            }
            return false;
        }

        Maze(usize size, u32 seed) :
            grid(size),
            draw_ctx(this->rotations, this->colors, this->grid)
        {
            DEBUGLN("Creating maze");
            ASSERT(size > 1);
            this->segments.reserve(size * size);
            this->rotations.reserve(size * size);

            std::default_random_engine rng(seed);
            std::uniform_real_distribution<GLfloat> dist(0.0f, 1.0f);

            const GLfloat length = 2.0f / (f32)size;
            this->colors[0] = { 0.824f, 0.106f, 0.106f };
            this->colors[1] = { 0.859f, 0.816f, 0.816f };

            this->draw_ctx.colors = this->colors;

            for (usize col_index = 0; col_index < size; ++col_index) {
                for (usize row_index = 0; row_index < size; ++row_index) {
                    const GLfloat angle = dist(rng) * M_PI;
                    glm::vec2 translation = grid.get_tail_center(col_index, row_index);
                    DEBUGFMTLN("Instance: %d, angle: %f", col_index * size + row_index, angle);
                    this->rotations.push_back(angle);

                    glm::vec2 offset = {0.0, 0.0};
                    if ((col_index == 0 && row_index == 0) || (row_index == size - 1 && col_index == size - 1)) {
                        offset += glm::vec2(2.0, 2.0);
                    }

                    Segment segment = grid.get_sample_segment();
                    segment.rotate(angle);
                    segment.move(translation + offset);
                    fprintf(stderr, "[TEST ] :: (%f, %f), (%f, %f)\n", segment.p1.x, segment.p1.y, segment.p2.x, segment.p2.y);
                    this->segments.push_back(segment);
                }
            }

            this->start();
        }

        virtual void register_shaders() override {
            this->gl_manager.compile_shaders_from_file("shaders/maze_v.glsl", "shaders/maze_f.glsl");
        }

        virtual void register_buffers() override {
            DEBUGLN("Loading Maze model data:");
            DEBUGFMTLN("-- vbo: %d", this->gl_manager.vbos[Maze::BUFFER_ID_MODEL]);

            this->gl_manager.bind_buffer<Maze::BUFFER_ID_MODEL>();
            DEBUGFMTLN("-- (%f, %f), (%f, %f)", this->draw_ctx.model.p1.x, this->draw_ctx.model.p1.y, this->draw_ctx.model.p2.x, this->draw_ctx.model.p2.y);
            glBufferData(
                GL_ARRAY_BUFFER,
                sizeof(this->draw_ctx.model),
                &this->draw_ctx.model,
                GL_STATIC_DRAW
            ); ASSERTGL();

            DEBUGLN("Configuring attribute pointer for Maze Model:");
            DEBUGFMTLN("-- layout: %d", Maze::SHADER_ATTRIBUTE_ID_MODEL);
            glEnableVertexAttribArray(Maze::SHADER_ATTRIBUTE_ID_MODEL); ASSERTGL();
            glVertexAttribPointer(
                Maze::SHADER_ATTRIBUTE_ID_MODEL,
                2,
                GL_FLOAT,
                GL_FALSE,
                2 * sizeof(GLfloat),
                (void*)0
            ); ASSERTGL();

            DEBUGLN("Loading Maze color data:");
            DEBUGFMTLN("-- vbo: %d", this->gl_manager.vbos[Maze::BUFFER_ID_COLOR]);
            glEnableVertexAttribArray(Maze::SHADER_ATTRIBUTE_ID_COLOR); ASSERTGL();
            this->gl_manager.bind_buffer<Maze::BUFFER_ID_COLOR>();
            
            glBufferData(
                GL_ARRAY_BUFFER,
                this->draw_ctx.colors.size() * sizeof(this->draw_ctx.colors.data()[0]),
                this->draw_ctx.colors.data(),
                GL_STATIC_DRAW
            ); ASSERTGL();

            DEBUGLN("Configuring attribute pointer for Maze color:");
            DEBUGFMTLN("-- layout: %d", Maze::SHADER_ATTRIBUTE_ID_COLOR);
            glEnableVertexAttribArray(Maze::SHADER_ATTRIBUTE_ID_COLOR); ASSERTGL();
            glVertexAttribPointer(
                Maze::SHADER_ATTRIBUTE_ID_COLOR,
                3,
                GL_FLOAT,
                GL_FALSE,
                3 * sizeof(GLfloat),
                (void*)0
            ); ASSERTGL();

            DEBUGLN("Loading Maze rotation data:");
            DEBUGFMTLN("-- vbo: %d", this->gl_manager.vbos[Maze::BUFFER_ID_ROTATION]);

            glEnableVertexAttribArray(Maze::SHADER_ATTRIBUTE_ID_ROTATION); ASSERTGL();
            this->gl_manager.bind_buffer<Maze::BUFFER_ID_ROTATION>();
            glBufferData(
                GL_ARRAY_BUFFER,
                this->draw_ctx.rotations.size() * sizeof(this->draw_ctx.rotations.data()[0]),
                this->draw_ctx.rotations.data(),
                GL_STATIC_DRAW
            ); ASSERTGL();

            DEBUGLN("Configuring attribute pointer for Maze rotation:");
            DEBUGFMTLN("-- layout: %d", Maze::SHADER_ATTRIBUTE_ID_ROTATION);
            glEnableVertexAttribArray(Maze::SHADER_ATTRIBUTE_ID_ROTATION); ASSERTGL();
            glVertexAttribPointer(
                Maze::SHADER_ATTRIBUTE_ID_ROTATION,
                1,
                GL_FLOAT,
                GL_FALSE,
                1 * sizeof(GLfloat),
                (void*)0
            ); ASSERTGL();
            glVertexAttribDivisor(Maze::SHADER_ATTRIBUTE_ID_ROTATION, 1);  ASSERTGL();
        }

        void draw() override {
            DEBUGLN("Issuing instanced draw call");
            glUniform1i(0, (int)this->grid.row_tail_count);
            DEBUGFMTLN("Maze segment count: %d", this->segments.size());
            glDrawArraysInstanced(GL_LINES, 0, 2, this->segments.size()); ERRORGL("Instanced drawing failed");
        }

        void bind_shaders() override {
            this->gl_manager.bind_program();
        }

        void bind_vao() override {
            this->gl_manager.bind_vao();
        }

        void unbind_vao() override {
            this->gl_manager.unbind_vao();
        }

        bool doCollide(const Segment& other) const {
            for (auto segment : this->segments) {
                if (collisions::doCollide(segment, other)) {
                    return true;
                }
            }
            return false;
        }
    };
    
    struct Player : components::IDrawable<> {
        components::StateManager<> gl_manager;
        const Segment model;
        glm::vec3 color;
        GLfloat angle;
        glm::vec2 offset;


        Player(Segment model, glm::vec2 starting_position) : model(model), color(0.0, 0.0, 1.0) {
            DEBUGLN("Creating player");
            this->start();
            offset = starting_position;
        }

        Segment get_model() const {
            Segment val = this->model;
            const glm::mat2 rotation = {
                { std::cos(angle), -std::sin(angle) },
                { std::sin(angle),  std::cos(angle) }
            };
            val.p1 = rotation * val.p1;
            val.p2 = rotation * val.p2;
            val.p1 += offset;
            val.p2 += offset;
            return val;
        }

        void register_shaders() override {
            this->gl_manager.compile_shaders_from_file("shaders/player_v.glsl", "shaders/player_f.glsl");
        }

        void register_buffers() override {
            this->gl_manager.bind_buffer<>();
            glBufferData(GL_ARRAY_BUFFER, sizeof(this->model), &this->model, GL_STATIC_DRAW); ASSERTGL();
            glEnableVertexAttribArray(0); ASSERTGL();
            glVertexAttribPointer(
                0,                  // attribute 0, must match the layout in the shader.
                2,                  // size
                GL_FLOAT,           // type
                GL_FALSE,           // normalized?
                0,                  // stride
                (void*)0            // array buffer offset
            ); ASSERTGL();
        }

        void draw() override {
            printf("Angle: %f\n", angle);
            glm::mat2 rotation = {
                { std::cos(angle), -std::sin(angle) },
                { std::sin(angle),  std::cos(angle) }
            };
            glUniform2f(1, this->offset.x, this->offset.y);
            glUniformMatrix2fv(2, 1, GL_FALSE, (GLfloat*)&rotation);
            glUniform3f(3, this->color.r, this->color.g, this->color.b);
            glDrawArrays(GL_LINES, 0, 2);
        }
        virtual void bind_vao() override {
            this->gl_manager.bind_vao();
        }
        virtual void unbind_vao() override {
            this->gl_manager.unbind_vao();
        }
        virtual void bind_shaders() override {
            this->gl_manager.bind_program();
        }
    };

    struct Animation : public components::IDrawable<0> {
        components::StateManager<0> gl_manager;

        Animation() {
            this->start();
        }

        void register_shaders() override {
            this->gl_manager.compile_shaders_from_file("shaders/background_v.glsl", "shaders/background_f.glsl"); ASSERTGL();
        }
        void register_buffers() override { }
        void draw() override {
            GLint m_viewport[4];
            glGetIntegerv(GL_VIEWPORT, m_viewport); ASSERTGL();

            glUniform2f(0, (float)m_viewport[2], (float)m_viewport[3]); ASSERTGL();
            glUniform1f(1, (float)glfwGetTime() * 0.001); ASSERTGL();
            glUniform1f(2, 0.5f); ASSERTGL();
            glUniform1f(3, 0.01f); ASSERTGL();
            glUniform2f(4, 1.0f, 1.0f); ASSERTGL();

            glDrawArrays(GL_TRIANGLE_STRIP, 0, 4); ERRORGL("Animation Draw failed");
        }
        void bind_vao() override {
            this->gl_manager.bind_vao();
        }
        void unbind_vao() override {
            this->gl_manager.unbind_vao();
        }
        void bind_shaders() override {
            this->gl_manager.bind_program();
        }
    };
}

class MyWin : public AGLWindow {
public:
    MyWin() {};
    MyWin(int _wd, int _ht, const char *name, int vers, int fullscr=0)
        : AGLWindow(_wd, _ht, name, vers, fullscr) {};
    virtual void KeyCB(int key, int scancode, int action, int mods);
    void MainLoop();
};



void MyWin::KeyCB(int key, int scancode, int action, int mods) {
    AGLWindow::KeyCB(key,scancode, action, mods); // f-key full screen switch
    if ((key == GLFW_KEY_SPACE) && action == GLFW_PRESS) {
       ; // do something
    }
    if (key == GLFW_KEY_HOME  && (action == GLFW_PRESS)) {
       ; // do something
    }
}


void MyWin::MainLoop() {
    ViewportOne(0, 0, wd, ht);
    glClearColor(0.157, 0.157, 0.157, 1.0);
    
    const glm::vec2 velocity = glm::vec2(0.01f, 0.01f);
    const GLfloat angular_velocity = 0.01f;

    _2d::Maze maze = _2d::Maze(5, 0);
    _2d::Animation animation = _2d::Animation();
    _2d::Player player = _2d::Player(maze.grid.get_sample_segment(), maze.grid.player_start_position());

    _2d::Segment vertical;
    vertical.p1 = glm::vec2(0.0f,  1.0f);
    vertical.p2 = glm::vec2(0.0f, -1.0f);

    _2d::Segment horizontal;
    horizontal.p1 = glm::vec2( 1.0f, 0.0f);
    horizontal.p2 = glm::vec2(-1.0f, 0.0f);

    do {
        glClear( GL_COLOR_BUFFER_BIT );
   
        ERRORGL("Error before draw loop");
        animation.draw_call();
        maze.draw_call();
        player.draw_call();
        ERRORGL("Error after draw loop");

        glfwSwapBuffers(win());
        glfwPollEvents();

        _2d::Segment player_hitbox = player.get_model();

        const bool doCollide = maze.check_collisions(player_hitbox);
        if (doCollide) {
            printf("Collision\n");
            player.color = { 1.0, 0.0, 0.0 };
        }
        else {
            player.color = { 0.0, 0.0, 1.0 };
        }

        if (glfwGetKey(win(), GLFW_KEY_DOWN ) == GLFW_PRESS) {
            DEBUGLN("DOWN");
            player.offset += glm::vec2(0.0f, -0.01f);
        } else if (glfwGetKey(win(), GLFW_KEY_UP ) == GLFW_PRESS) {
            DEBUGLN("UP");
            player.offset += glm::vec2(0.0f, 0.01f);
        } else if (glfwGetKey(win(), GLFW_KEY_RIGHT ) == GLFW_PRESS) {
            DEBUGLN("RIGHT");
            player.offset += glm::vec2(0.01f, 0.0f);
        } else if (glfwGetKey(win(), GLFW_KEY_LEFT ) == GLFW_PRESS) { 
            DEBUGLN("LEFT");
            player.offset += glm::vec2(-0.01f, 0.0f);
        } else if (glfwGetKey(win(), GLFW_KEY_E) == GLFW_PRESS) {
            DEBUGLN("E");
            player.angle += 0.01;
        } else if (glfwGetKey(win(), GLFW_KEY_Q) == GLFW_PRESS) {
            DEBUGLN("Q");
            player.angle -= 0.01;
        }
    } while(
        glfwGetKey(win(), GLFW_KEY_ESCAPE) != GLFW_PRESS && 
        glfwWindowShouldClose(win()) == 0
    );
}

int main(int argc, char *argv[]) {
   MyWin win;
   win.Init(800, 600, "AGL3 example", 0, 33);

   glewExperimental = GL_TRUE;
   if (glewInit() != GLEW_OK) {
       fprintf(stderr, "Failed to initialize GLEW\n");
       getchar();
       glfwTerminate();
       return -1;
   }
   win.MainLoop();
   int press_to_exit;
   std::cin >> press_to_exit;
   return EXIT_SUCCESS;
}
