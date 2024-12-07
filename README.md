# RSCMS (Rust Content Management System)

---

[English](./README.md) | [中文](./README_cn.md)

**Introduction:**

RSCMS is a cutting-edge open-source content management system built entirely in Rust, designed to empower developers with a robust and efficient platform for managing digital content. Leveraging the power and performance of Rust, RSCMS offers a modern solution for creating, organizing, and delivering content across a wide range of applications and websites.

**Key Features:**

1. **Rust-Powered Performance:** RSCMS harnesses the speed and reliability of Rust to deliver high-performance content management capabilities, ensuring optimal efficiency and scalability.

2. **Modular Architecture:** With a flexible and modular architecture, developers can easily extend and customize RSCMS to suit their specific content management needs.

3. **User-Friendly Interface:** RSCMS provides an intuitive and user-friendly interface for content creation, editing, and publishing, making it easy for users to manage their digital content effectively.

4. **Security and Reliability:** Built with security in mind, RSCMS prioritizes data protection and system integrity, offering a secure environment for managing sensitive content.

5. **Community-Driven Development:** RSCMS is an open-source project, welcoming contributions from the community to enhance features, fix bugs, and drive continuous improvement.

**Getting Started:**

Prerequisites:
- Rust (latest stable version)
- Docker and Docker Compose
- Git

Follow these steps to set up and run RSCMS:

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/rscms-dev/rscms.git
   cd rscms
   ```

2. **Configure Environment Variables:**
   Create a `.env` file in the project root with the following variables:
   ```env
   # Server Configuration
   SERVER_HOST=127.0.0.1    # Server host address
   SERVER_PORT=3000         # Server port number

   # Database Configuration
   DATABASE_URL=mysql://root:example@localhost:3306/rscms

   # JWT Configuration (for authentication)
   JWT_SECRET=your-super-secret-and-ultra-long-secret-key

   # Email Configuration (optional)
   SMTP_HOST=smtp.example.com
   SMTP_PORT=587
   SMTP_USERNAME=your-username
   SMTP_PASSWORD=your-password
   SMTP_FROM_EMAIL=noreply@example.com
   ```

3. **Start the Development Database:**
   ```bash
   cd db
   docker-compose up -d
   ```
   This will start:
   - MySQL database (accessible at localhost:3306)
   - PHPMyAdmin interface (accessible at http://localhost:8080)
   
   Default database credentials:
   - Username: root
   - Password: rscms
   - Database: rscms

4. **Build and Run RSCMS:**
   ```bash
   cargo build
   cargo run
   ```

5. **Access the API:**
   The API will be available at `http://{SERVER_HOST}:{SERVER_PORT}`
   
   Note: RSCMS currently provides API endpoints only. There is no web interface available yet.

**Development Database Management:**
- PHPMyAdmin is available at http://localhost:8080
- To stop the database: `docker-compose down`
- To view database logs: `docker-compose logs`
- Database data is persisted in `./db/data`
- Initial migrations are automatically applied from `./db/migrations`

**Get Involved:**

Join us in shaping the future of content management with Rust! Whether you're a developer looking to contribute code, a designer interested in improving the user experience, or a content creator seeking a modern CMS solution, there are opportunities for everyone to get involved and make a difference in the RSCMS project.

**Explore RSCMS Today:**

Visit our GitHub organization "rscms-dev" to access the latest codebase, contribute to the project, and join a vibrant community of developers passionate about modern content management in Rust.
