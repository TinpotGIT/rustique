/**
 * Rustique V2 - Main JavaScript
 * Developed for the Rustique digital painting application website
 */

// DOM Elements
const loader = document.querySelector('.loader');
const progress = document.querySelector('.progress');
const cursor = document.querySelector('.cursor');
const cursorFollower = document.querySelector('.cursor-follower');
const header = document.querySelector('.header');
const mobileMenuBtn = document.querySelector('.mobile-menu-btn');
const hamburger = document.querySelector('.hamburger');
const navLinks = document.querySelector('.nav-links');
const navLinksItems = document.querySelectorAll('.nav-link');
const themeToggle = document.getElementById('theme-toggle');
const tabBtns = document.querySelectorAll('.tab-btn');
const tabPanes = document.querySelectorAll('.tab-pane');
const backToTopBtn = document.querySelector('.back-to-top');
const newsletterForm = document.querySelector('.newsletter-form');
const counterItems = document.querySelectorAll('.counter-number');
const scrollLinks = document.querySelectorAll('[data-scroll]');

/**
 * Page Preloader
 */
const preloader = (() => {
    let loadedCount = 0;
    const totalItems = document.images.length + 2; // Images + CSS + JS
    
    const increment = () => {
        loadedCount++;
        const percentage = (loadedCount / totalItems) * 100;
        progress.style.width = `${percentage}%`;
        
        if (loadedCount >= totalItems) {
            setTimeout(() => {
                loader.classList.add('fade-out');
                document.body.style.overflow = 'visible';
                animateHeroContent();
            }, 500);
        }
    };
    
    const init = () => {
        // Initialize progress bar
        progress.style.width = '0%';
        
        // Count loaded images
        if (document.images.length > 0) {
            for (let img of document.images) {
                if (img.complete) {
                    increment();
                } else {
                    img.addEventListener('load', increment);
                    img.addEventListener('error', increment);
                }
            }
        } else {
            increment();
            increment();
        }
        
        // Count CSS and JS
        increment();
        increment();
    };
    
    return {
        init
    };
})();

/**
 * Custom Cursor
 */
const customCursor = (() => {
    let mouseX = 0;
    let mouseY = 0;
    let cursorX = 0;
    let cursorY = 0;
    let followerX = 0;
    let followerY = 0;
    
    const init = () => {
        if (window.innerWidth <= 768) return;
        
        // Show cursor elements
        setTimeout(() => {
            cursor.classList.add('active');
            cursorFollower.classList.add('active');
        }, 1000);
        
        // Track mouse position
        document.addEventListener('mousemove', (e) => {
            mouseX = e.clientX;
            mouseY = e.clientY;
        });
        
        // Handle hover states
        document.querySelectorAll('a, button, [role="button"], input, label, .feature-card, .team-member, .download-btn, .info-item')
            .forEach(el => {
                el.addEventListener('mouseenter', () => {
                    cursorFollower.classList.add('hover');
                });
                
                el.addEventListener('mouseleave', () => {
                    cursorFollower.classList.remove('hover');
                });
            });
        
        // Start animation
        animateCursor();
    };
    
    const animateCursor = () => {
        const render = () => {
            // Smoothly follow the mouse position
            cursorX += (mouseX - cursorX) * 0.2;
            cursorY += (mouseY - cursorY) * 0.2;
            followerX += (mouseX - followerX) * 0.1;
            followerY += (mouseY - followerY) * 0.1;
            
            // Apply transformations
            cursor.style.transform = `translate(${cursorX}px, ${cursorY}px)`;
            cursorFollower.style.transform = `translate(${followerX}px, ${followerY}px)`;
            
            requestAnimationFrame(render);
        };
        
        requestAnimationFrame(render);
    };
    
    return {
        init
    };
})();

/**
 * Mobile Menu Toggle
 */
const mobileMenu = (() => {
    const init = () => {
        mobileMenuBtn.addEventListener('click', () => {
            hamburger.classList.toggle('active');
            navLinks.classList.toggle('active');
            document.body.classList.toggle('menu-open');
        });
        
        // Close mobile menu when clicking on a navigation link
        navLinksItems.forEach(link => {
            link.addEventListener('click', () => {
                hamburger.classList.remove('active');
                navLinks.classList.remove('active');
                document.body.classList.remove('menu-open');
            });
        });
    };
    
    return {
        init
    };
})();

/**
 * Theme Switcher
 */
const themeSwitcher = (() => {
    const toggleTheme = () => {
        const isDarkMode = themeToggle.checked;
        document.documentElement.setAttribute('data-theme', isDarkMode ? 'dark' : 'light');
        localStorage.setItem('theme', isDarkMode ? 'dark' : 'light');
    };
    
    const init = () => {
        // Set theme from local storage or default to dark
        const savedTheme = localStorage.getItem('theme') || 'dark';
        document.documentElement.setAttribute('data-theme', savedTheme);
        themeToggle.checked = savedTheme === 'dark';
        
        // Listen for theme changes
        themeToggle.addEventListener('change', toggleTheme);
    };
    
    return {
        init
    };
})();

/**
 * Scroll Effects
 */
const scrollEffects = (() => {
    const handleScroll = () => {
        const scrollPosition = window.scrollY;
        
        // Header shrink effect
        if (scrollPosition > 50) {
            header.classList.add('scrolled');
        } else {
            header.classList.remove('scrolled');
        }
        
        // Back to top button visibility
        if (scrollPosition > 500) {
            backToTopBtn.classList.add('active');
        } else {
            backToTopBtn.classList.remove('active');
        }
        
        // Start counter animation when in viewport
        animateCounters();
    };
    
    const setupSmoothScroll = () => {
        scrollLinks.forEach(link => {
            link.addEventListener('click', function(e) {
                e.preventDefault();
                
                const targetId = this.getAttribute('href');
                const targetElement = document.querySelector(targetId);
                
                if (targetElement) {
                    const offsetTop = targetElement.offsetTop - 100;
                    
                    window.scrollTo({
                        top: offsetTop,
                        behavior: 'smooth'
                    });
                }
            });
        });
    };
    
    const animateCounters = () => {
        counterItems.forEach(counter => {
            const counterPosition = counter.getBoundingClientRect().top;
            const windowHeight = window.innerHeight;
            
            if (counterPosition < windowHeight && !counter.classList.contains('animated')) {
                counter.classList.add('animated');
                
                const target = parseInt(counter.getAttribute('data-target'));
                let count = 0;
                const duration = 2000; // 2 seconds
                const interval = Math.floor(duration / target);
                
                const counterAnimation = setInterval(() => {
                    count++;
                    counter.textContent = count;
                    
                    if (count >= target) {
                        clearInterval(counterAnimation);
                    }
                }, Math.max(interval, 10));
            }
        });
    };
    
    const init = () => {
        window.addEventListener('scroll', handleScroll);
        setupSmoothScroll();
        
        // Initialize header state
        handleScroll();
    };
    
    return {
        init
    };
})();

/**
 * Feature Tabs
 */
const featureTabs = (() => {
    const init = () => {
        tabBtns.forEach(btn => {
            btn.addEventListener('click', () => {
                const tabId = btn.getAttribute('data-tab');
                
                // Remove active class from all buttons and panes
                tabBtns.forEach(b => b.classList.remove('active'));
                tabPanes.forEach(p => p.classList.remove('active'));
                
                // Add active class to clicked button and corresponding pane
                btn.classList.add('active');
                document.getElementById(tabId).classList.add('active');
            });
        });
    };
    
    return {
        init
    };
})();

/**
 * Animations
 */
const animations = (() => {
    const initAOS = () => {
        AOS.init({
            duration: 800,
            easing: 'ease-in-out',
            once: true,
            mirror: false
        });
    };
    
    const initGSAP = () => {
        // Register GSAP plugins
        gsap.registerPlugin(ScrollTrigger);
        
        // Timeline animations
        gsap.from('.timeline-item', {
            opacity: 0,
            y: 50,
            stagger: 0.2,
            duration: 1,
            ease: 'power3.out',
            scrollTrigger: {
                trigger: '.timeline',
                start: 'top 70%'
            }
        });
        
        // Feature cards animations
        gsap.from('.feature-card', {
            opacity: 0,
            y: 30,
            stagger: 0.1,
            duration: 0.8,
            ease: 'back.out(1.7)',
            scrollTrigger: {
                trigger: '.features-grid',
                start: 'top 80%'
            }
        });
    };
    
    const animateHeroContent = () => {
        const tl = gsap.timeline();
        
        tl.from('.hero-title', {
            opacity: 0,
            y: 30,
            duration: 1,
            ease: 'power3.out'
        })
        .from('.hero-subtitle', {
            opacity: 0,
            y: 30,
            duration: 1,
            ease: 'power3.out'
        }, '-=0.6')
        .from('.hero-btns', {
            opacity: 0,
            y: 30,
            duration: 1,
            ease: 'power3.out'
        }, '-=0.6')
        .from('.hero-image', {
            opacity: 0,
            y: 30,
            duration: 1,
            ease: 'power3.out'
        }, '-=0.6')
        .from('.scroll-down', {
            opacity: 0,
            y: 30,
            duration: 1,
            ease: 'power3.out'
        }, '-=0.4');
    };
    
    const init = () => {
        initAOS();
        initGSAP();
    };
    
    return {
        init,
        animateHeroContent
    };
})();

/**
 * Particles.js Background
 */
const particlesBackground = (() => {
    const init = () => {
        if (document.getElementById('particles-js')) {
            particlesJS('particles-js', {
                "particles": {
                    "number": {
                        "value": 80,
                        "density": {
                            "enable": true,
                            "value_area": 800
                        }
                    },
                    "color": {
                        "value": "#CF6A37"
                    },
                    "shape": {
                        "type": "circle",
                        "stroke": {
                            "width": 0,
                            "color": "#000000"
                        },
                        "polygon": {
                            "nb_sides": 5
                        }
                    },
                    "opacity": {
                        "value": 0.5,
                        "random": false,
                        "anim": {
                            "enable": false,
                            "speed": 1,
                            "opacity_min": 0.1,
                            "sync": false
                        }
                    },
                    "size": {
                        "value": 3,
                        "random": true,
                        "anim": {
                            "enable": false,
                            "speed": 40,
                            "size_min": 0.1,
                            "sync": false
                        }
                    },
                    "line_linked": {
                        "enable": true,
                        "distance": 150,
                        "color": "#CF6A37",
                        "opacity": 0.4,
                        "width": 1
                    },
                    "move": {
                        "enable": true,
                        "speed": 2,
                        "direction": "none",
                        "random": false,
                        "straight": false,
                        "out_mode": "out",
                        "bounce": false,
                        "attract": {
                            "enable": false,
                            "rotateX": 600,
                            "rotateY": 1200
                        }
                    }
                },
                "interactivity": {
                    "detect_on": "canvas",
                    "events": {
                        "onhover": {
                            "enable": true,
                            "mode": "grab"
                        },
                        "onclick": {
                            "enable": true,
                            "mode": "push"
                        },
                        "resize": true
                    },
                    "modes": {
                        "grab": {
                            "distance": 140,
                            "line_linked": {
                                "opacity": 1
                            }
                        },
                        "bubble": {
                            "distance": 400,
                            "size": 40,
                            "duration": 2,
                            "opacity": 8,
                            "speed": 3
                        },
                        "repulse": {
                            "distance": 200,
                            "duration": 0.4
                        },
                        "push": {
                            "particles_nb": 4
                        },
                        "remove": {
                            "particles_nb": 2
                        }
                    }
                },
                "retina_detect": true
            });
        }
    };
    
    return {
        init
    };
})();

/**
 * Newsletter Form
 */
const newsletter = (() => {
    const init = () => {
        if (newsletterForm) {
            newsletterForm.addEventListener('submit', (e) => {
                e.preventDefault();
                
                const emailInput = newsletterForm.querySelector('input[type="email"]');
                const email = emailInput.value;
                
                if (email) {
                    // Show success message (in production, you'd send data to a server)
                    emailInput.value = '';
                    alert('Merci de vous être abonné à notre newsletter !');
                }
            });
        }
    };
    
    return {
        init
    };
})();

/**
 * Active Navigation Link
 */
const activeNavigation = (() => {
    const sections = document.querySelectorAll('section[id]');
    
    const highlightNavLink = () => {
        const scrollY = window.scrollY;
        
        sections.forEach(section => {
            const sectionHeight = section.offsetHeight;
            const sectionTop = section.offsetTop - 100;
            const sectionId = section.getAttribute('id');
            
            if (scrollY > sectionTop && scrollY <= sectionTop + sectionHeight) {
                document.querySelector(`.nav-link[href="#${sectionId}"]`)?.classList.add('active');
            } else {
                document.querySelector(`.nav-link[href="#${sectionId}"]`)?.classList.remove('active');
            }
        });
    };
    
    const init = () => {
        window.addEventListener('scroll', highlightNavLink);
        highlightNavLink(); // Call once on init
    };
    
    return {
        init
    };
})();

/**
 * Initialization
 */
document.addEventListener('DOMContentLoaded', () => {
    // Initialize modules
    preloader.init();
    customCursor.init();
    mobileMenu.init();
    themeSwitcher.init();
    scrollEffects.init();
    featureTabs.init();
    animations.init();
    particlesBackground.init();
    newsletter.init();
    activeNavigation.init();
    
    // Expose animateHeroContent function to window for the preloader
    window.animateHeroContent = animations.animateHeroContent;
});