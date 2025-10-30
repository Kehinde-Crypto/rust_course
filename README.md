<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Programming Course</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            color: #fff;
            overflow-x: hidden;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }

        .hero {
            text-align: center;
            padding: 60px 20px;
            position: relative;
            overflow: hidden;
        }

        .rust-logo {
            font-size: 100px;
            margin-bottom: 20px;
            animation: float 3s ease-in-out infinite;
        }

        @keyframes float {
            0%, 100% { transform: translateY(0px); }
            50% { transform: translateY(-20px); }
        }

        h1 {
            font-size: 3em;
            margin-bottom: 20px;
            background: linear-gradient(45deg, #ff6b6b, #ee5a6f, #f06595);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            animation: slideDown 1s ease-out;
        }

        @keyframes slideDown {
            from {
                opacity: 0;
                transform: translateY(-50px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .subtitle {
            font-size: 1.3em;
            color: #a8b2d1;
            margin-bottom: 40px;
            animation: fadeIn 1.5s ease-out;
        }

        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }

        .module-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 30px;
            margin: 60px 0;
        }

        .module-card {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 20px;
            padding: 30px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            transition: all 0.3s ease;
            opacity: 0;
            transform: translateY(30px);
            animation: cardSlideUp 0.6s ease-out forwards;
        }

        .module-card:nth-child(1) { animation-delay: 0.1s; }
        .module-card:nth-child(2) { animation-delay: 0.2s; }
        .module-card:nth-child(3) { animation-delay: 0.3s; }
        .module-card:nth-child(4) { animation-delay: 0.4s; }
        .module-card:nth-child(5) { animation-delay: 0.5s; }
        .module-card:nth-child(6) { animation-delay: 0.6s; }
        .module-card:nth-child(7) { animation-delay: 0.7s; }
        .module-card:nth-child(8) { animation-delay: 0.8s; }

        @keyframes cardSlideUp {
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .module-card:hover {
            transform: translateY(-10px) scale(1.02);
            background: rgba(255, 255, 255, 0.1);
            border-color: #ff6b6b;
            box-shadow: 0 20px 40px rgba(255, 107, 107, 0.3);
        }

        .module-icon {
            font-size: 2.5em;
            margin-bottom: 15px;
            display: inline-block;
            transition: transform 0.3s ease;
        }

        .module-card:hover .module-icon {
            transform: rotate(360deg) scale(1.2);
        }

        .module-title {
            font-size: 1.5em;
            margin-bottom: 10px;
            color: #ff6b6b;
        }

        .module-desc {
            color: #a8b2d1;
            line-height: 1.6;
        }

        .section {
            margin: 80px 0;
        }

        .section-title {
            font-size: 2.5em;
            text-align: center;
            margin-bottom: 40px;
            position: relative;
        }

        .section-title::after {
            content: '';
            display: block;
            width: 100px;
            height: 4px;
            background: linear-gradient(90deg, #ff6b6b, #ee5a6f);
            margin: 20px auto;
            border-radius: 2px;
        }

        .feature-list {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            justify-content: center;
        }

        .feature-item {
            background: rgba(255, 255, 255, 0.05);
            padding: 20px 30px;
            border-radius: 50px;
            border: 1px solid rgba(255, 255, 255, 0.1);
            transition: all 0.3s ease;
            cursor: default;
        }

        .feature-item:hover {
            background: rgba(255, 107, 107, 0.2);
            border-color: #ff6b6b;
            transform: scale(1.05);
        }

        .cta-section {
            text-align: center;
            padding: 60px 20px;
            background: rgba(255, 107, 107, 0.1);
            border-radius: 30px;
            margin: 60px 0;
            border: 2px solid rgba(255, 107, 107, 0.3);
        }

        .cta-button {
            display: inline-block;
            padding: 20px 50px;
            background: linear-gradient(45deg, #ff6b6b, #ee5a6f);
            color: white;
            text-decoration: none;
            border-radius: 50px;
            font-size: 1.2em;
            font-weight: bold;
            transition: all 0.3s ease;
            margin-top: 20px;
            border: none;
            cursor: pointer;
            box-shadow: 0 10px 30px rgba(255, 107, 107, 0.4);
        }

        .cta-button:hover {
            transform: translateY(-5px);
            box-shadow: 0 15px 40px rgba(255, 107, 107, 0.6);
        }

        .stats-container {
            display: flex;
            justify-content: space-around;
            flex-wrap: wrap;
            gap: 40px;
            margin: 60px 0;
        }

        .stat-box {
            text-align: center;
        }

        .stat-number {
            font-size: 3em;
            font-weight: bold;
            color: #ff6b6b;
            margin-bottom: 10px;
        }

        .stat-label {
            color: #a8b2d1;
            font-size: 1.1em;
        }

        .particles {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            pointer-events: none;
            z-index: -1;
        }

        .particle {
            position: absolute;
            width: 4px;
            height: 4px;
            background: #ff6b6b;
            border-radius: 50%;
            animation: particleFloat 20s infinite;
        }

        @keyframes particleFloat {
            0%, 100% {
                transform: translateY(0) translateX(0);
                opacity: 0;
            }
            10% {
                opacity: 1;
            }
            90% {
                opacity: 1;
            }
            100% {
                transform: translateY(-100vh) translateX(100px);
                opacity: 0;
            }
        }
    </style>
</head>
<body>
    <div class="particles" id="particles"></div>
    
    <div class="container">
        <div class="hero">
            <div class="rust-logo">🦀</div>
            <h1>Rust Programming Course</h1>
            <p class="subtitle">Master the language that powers performance, safety, and concurrency</p>
        </div>

        <div class="stats-container">
            <div class="stat-box">
                <div class="stat-number">8</div>
                <div class="stat-label">Modules</div>
            </div>
            <div class="stat-box">
                <div class="stat-number">50+</div>
                <div class="stat-label">Hours of Content</div>
            </div>
            <div class="stat-box">
                <div class="stat-number">30+</div>
                <div class="stat-label">Projects</div>
            </div>
        </div>

        <section class="section">
            <h2 class="section-title">Course Modules</h2>
            <div class="module-grid">
                <div class="module-card">
                    <div class="module-icon">🚀</div>
                    <h3 class="module-title">Getting Started</h3>
                    <p class="module-desc">Install Rust, set up your environment, and write your first program. Learn the basics of syntax and toolchain.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">🔒</div>
                    <h3 class="module-title">Ownership & Borrowing</h3>
                    <p class="module-desc">Master Rust's unique ownership system, references, and memory safety guarantees.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">📦</div>
                    <h3 class="module-title">Structs & Enums</h3>
                    <p class="module-desc">Define custom types, implement methods, and leverage powerful pattern matching.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">📚</div>
                    <h3 class="module-title">Collections & Errors</h3>
                    <p class="module-desc">Work with vectors, strings, hash maps, and handle errors gracefully.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">🎯</div>
                    <h3 class="module-title">Generics & Traits</h3>
                    <p class="module-desc">Write flexible, reusable code with generics and trait-based polymorphism.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">🧪</div>
                    <h3 class="module-title">Testing & Docs</h3>
                    <p class="module-desc">Build reliable software with comprehensive testing and documentation practices.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">⚡</div>
                    <h3 class="module-title">Concurrency</h3>
                    <p class="module-desc">Harness the power of multi-threading and async programming safely.</p>
                </div>
                
                <div class="module-card">
                    <div class="module-icon">🛠️</div>
                    <h3 class="module-title">Real Projects</h3>
                    <p class="module-desc">Build CLI tools, web services, and production-ready applications.</p>
                </div>
            </div>
        </section>

        <section class="section">
            <h2 class="section-title">What You'll Learn</h2>
            <div class="feature-list">
                <div class="feature-item">✓ Memory Safety</div>
                <div class="feature-item">✓ Zero-Cost Abstractions</div>
                <div class="feature-item">✓ Thread Safety</div>
                <div class="feature-item">✓ Pattern Matching</div>
                <div class="feature-item">✓ Error Handling</div>
                <div class="feature-item">✓ Async/Await</div>
                <div class="feature-item">✓ Web Development</div>
                <div class="feature-item">✓ CLI Applications</div>
                <div class="feature-item">✓ Systems Programming</div>
            </div>
        </section>

        <div class="cta-section">
            <h2>Ready to Start Your Rust Journey?</h2>
            <p style="margin: 20px 0; font-size: 1.2em; color: #a8b2d1;">Join thousands of developers mastering the future of systems programming</p>
            <button class="cta-button" onclick="alert('🦀 Let\'s build something amazing with Rust!')">Get Started Now</button>
        </div>
    </div>

    <script>
        // Create floating particles
        const particlesContainer = document.getElementById('particles');
        for (let i = 0; i < 30; i++) {
            const particle = document.createElement('div');
            particle.className = 'particle';
            particle.style.left = Math.random() * 100 + '%';
            particle.style.animationDelay = Math.random() * 20 + 's';
            particle.style.animationDuration = (15 + Math.random() * 10) + 's';
            particlesContainer.appendChild(particle);
        }
    </script>
</body>
</html>
